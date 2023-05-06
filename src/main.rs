pub mod handler;
pub mod nodes;
pub mod types;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let machine = {
        // #[cfg(feature = "echo")]
        // nodes::echo::EchoNode::default()
        nodes::generate::GenerateNode::default()
    };

    handler::executor(machine)
}
