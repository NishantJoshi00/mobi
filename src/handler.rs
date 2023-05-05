use std::io;

use serde_json::{de::IoRead, StreamDeserializer};

use crate::types;

pub fn executor<'a>(machine: impl Machine) -> anyhow::Result<()>
where
{
    let stdin = std::io::stdin().lock();
    let input_stream = serde_json::Deserializer::from_reader(stdin).into_iter();
    let output_stream = std::io::stdout().lock();

    machine.handle(input_stream, output_stream)
}

pub type JsonStreamDe<'de, 'a> =
    StreamDeserializer<'de, IoRead<io::StdinLock<'a>>, types::Message<'de>>;

pub type JsonSer<'a> = io::StdoutLock<'a>;

pub trait Machine {
    fn run(&mut self, input: JsonStreamDe, output: JsonSer) -> anyhow::Result<()>;

    fn handle(mut self, input: JsonStreamDe, output: JsonSer) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        self.run(input, output)
    }
}
