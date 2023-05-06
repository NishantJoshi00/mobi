use anyhow::Context;

use crate::{
    handler::{self, Machine},
    types::{Body, Message},
    utils::{error::Consume, io_ops::JsonWrite},
};

///
/// Node Created for the Echo Challenge
///
#[derive(Default)]
pub struct EchoNode {
    node_name: Option<String>,
}

impl Machine for EchoNode {
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
            handler::State::Id { id } => self.node_name = Some(id.into()),
        }
    }
}

impl EchoNode {
    fn step<'a>(&mut self, input: Message<'a>) -> anyhow::Result<Message<'a>> {
        Ok(match input.body {
            Body::Echo { msg_id, echo } => Message {
                body: Body::EchoOk {
                    msg_id,
                    in_reply_to: msg_id,
                    echo,
                },
                src: input.dst,
                dst: input.src,
            },
            msg => anyhow::bail!("Invalid message received: {:?}", msg),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut node = EchoNode::default();
        let data = r#"{"src": "c1", "dest": "n1", "body": {"type": "echo", "msg_id": 1, "echo": "Please echo 35"}}"#;
        let msg = serde_json::from_str(data).unwrap();
        let out_msg = node.step(msg).unwrap();
        let out = serde_json::to_string(&out_msg).unwrap();
        assert_eq!(
            out,
            "{\"src\":\"n1\",\"dest\":\"c1\",\"body\":{\"type\":\"echo_ok\",\"msg_id\":1,\"in_reply_to\":1,\"echo\":\"Please echo 35\"}}"
        );
    }
}
