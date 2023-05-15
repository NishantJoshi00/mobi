use std::time::UNIX_EPOCH;

use anyhow::{bail, Context};
use sha256::digest;

use crate::{
    handler::{self, Machine},
    types::{self, Message},
    utils::{error::Consume, io_ops::JsonWrite},
};

#[derive(Default)]
pub struct GenerateNode {
    node_name: Option<String>,
}

impl Machine for GenerateNode {
    fn run(
        &mut self,
        mut inputs: handler::JsonStreamDe,
        mut output: handler::JsonSer,
    ) -> anyhow::Result<()> {
        inputs.try_for_each(|msg| {
            self.step(msg?)
                .context("Failed while handling request")
                .consume()
                .map(|value| value.write_to_writer(&mut output))
                .transpose()?;
            Ok(())
        })
    }

    fn set_state(&mut self, state: handler::State) {
        match state {
            handler::State::Id { id } => self.node_name = Some(id.to_string()),
        }
    }
}

impl GenerateNode {
    fn step<'a>(&mut self, input: Message<'a>) -> anyhow::Result<Message<'a>> {
        input.respond(|msg_id, body| match body {
            types::RequestBody::Generate {} => {
                let data = digest(format!("{} {}", msg_id, UNIX_EPOCH.elapsed()?.as_millis()));
                Ok(types::ResponseBody::GenerateOk {
                    id: format!(
                        "{}-{}",
                        self.node_name.as_ref().unwrap_or(&"0".to_string()),
                        data
                    )
                    .into(),
                })
            }
            msg => bail!("Invalid message received: {:?}", msg),
        })
    }
}
