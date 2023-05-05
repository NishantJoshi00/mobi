pub mod error {
    pub trait Consume<T> {
        fn consume(self) -> Option<T>;
    }

    impl<T> Consume<T> for anyhow::Result<T> {
        fn consume(self) -> Option<T> {
            match self {
                Ok(item) => Some(item),
                Err(inner_err) => {
                    eprintln!("{:?}", inner_err);
                    None
                }
            }
        }
    }
}

pub mod io_ops {
    use std::io::Write;

    use anyhow::Context;
    use serde::Serialize;

    pub trait JsonWrite {
        fn write_to_writer<W: Write>(&self, writer: &mut W) -> anyhow::Result<()>;
    }

    impl<S> JsonWrite for S
    where
        S: Serialize,
    {
        fn write_to_writer<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
            serde_json::to_writer(&mut *writer, self)
                .context("Failed while writing to `Writer`")?;
            Ok(writer.write_all(b"\n").context("Failed to add newline")?)
            // Ok(writer.flush().context("Failed while `Flush`")?)
        }
    }
}
