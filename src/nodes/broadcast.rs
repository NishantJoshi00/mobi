use std::{borrow::Cow, collections::HashSet};

use anyhow::{bail, Context};

use crate::{
    handler::Machine,
    types::{self, Message},
    utils::{error::Consume, io_ops::JsonWrite},
};

#[derive(Default)]
pub struct BroadcastNode {
    node_name: Option<String>,
    store: Vec<usize>,
    neighbors: HashSet<String>,
}

impl Machine for BroadcastNode {
    fn run(
        &mut self,
        mut inputs: crate::handler::JsonStreamDe,
        mut output: crate::handler::JsonSer,
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

    fn set_state(&mut self, state: crate::handler::State) {
        match state {
            crate::handler::State::Id { id } => self.node_name = Some(id.to_string()),
        }
    }
}

impl BroadcastNode {
    fn step<'a>(&mut self, input: Message<'a>) -> anyhow::Result<Message<'a>> {
        eprintln!("request: {:?}", input);
        let response = input.respond(|_msg_id, request| {
            Ok(match request {
                types::RequestBody::Broadcast { message } => {
                    self.store.push(message);
                    types::ResponseBody::BroadcastOk {}
                }
                types::RequestBody::Topology { topology } => {
                    self.neighbors = self
                        .node_name
                        .as_ref()
                        .and_then(|name| {
                            topology
                                .get::<Cow<'_, str>>(&name.into())
                                .map(|set| set.to_owned())
                        })
                        .unwrap_or(HashSet::new())
                        .into_iter()
                        .map(Into::into)
                        .collect();
                    types::ResponseBody::TopologyOk {}
                }
                types::RequestBody::Read {} => types::ResponseBody::ReadOk {
                    messages: self.store.clone(),
                },
                msg => bail!("Invalid message received: {:?}", msg),
            })
        });
        eprintln!("response: {:?}", response);
        response
    }
}
