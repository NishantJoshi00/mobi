use std::{borrow::Cow, io};

use anyhow::anyhow;
use serde_json::{de::IoRead, StreamDeserializer};

use crate::{types, utils::io_ops::JsonWrite};

/// An executor to provide input and output attachments for `Machine`
/// Converts `stdin` into `StreamDeserializer`
/// And provides `stdout` stream to `Machine` `handle`
pub fn executor(machine: impl Machine) -> anyhow::Result<()>
where
{
    eprintln!("[log] connecting to I/O");
    let stdin = std::io::stdin().lock();
    let input_stream = serde_json::Deserializer::from_reader(stdin).into_iter();
    let output_stream = std::io::stdout().lock();

    eprintln!("[log] executing handler");
    machine.handle(input_stream, output_stream)
}

pub type JsonStreamDe<'de, 'a> =
    StreamDeserializer<'de, IoRead<io::StdinLock<'a>>, types::Message<'de>>;

pub type JsonSer<'a> = io::StdoutLock<'a>;

/// Trait to provide abstraction for node, to define specific behaviour,
/// provides abstraction over how the machine should run and how the machine is executed.
/// Allows, one machine to execute multiple machines
pub trait Machine {
    fn run(&mut self, inputs: JsonStreamDe, output: JsonSer) -> anyhow::Result<()>;

    fn handle(mut self, mut input: JsonStreamDe, mut output: JsonSer) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        input
            .next()
            .map(|msg| {
                let handshake_out = self.handshake(msg?)?;
                handshake_out.write_to_writer(&mut output)
            })
            .transpose()?
            .ok_or(anyhow!("Failed while making handshake"))?;
        eprintln!("[log] handshake complete");
        self.run(input, output)
    }

    fn set_state(&mut self, state: State);

    fn handshake<'a>(&mut self, input: types::Message<'a>) -> anyhow::Result<types::Message<'a>> {
        eprintln!("handshake request: {:?}", input);
        let output = Ok(match input.body {
            types::Body::Request { msg_id, body } => match body {
                types::RequestBody::Init { node_id, .. } => {
                    self.set_state(State::Id { id: node_id });
                    types::Message {
                        src: input.dst,
                        dst: input.src,
                        body: types::Body::Response {
                            in_reply_to: msg_id,
                            body: types::ResponseBody::InitOk {},
                        },
                    }
                }
                msg => anyhow::bail!("Invalid message for handshake: {:?}", msg),
            },
            msg => anyhow::bail!("Invalid message for handshake: {:?}", msg),
        });
        eprintln!("handshake response: {:?}", output);
        output
    }
}

pub enum State<'a> {
    Id { id: Cow<'a, str> },
}
