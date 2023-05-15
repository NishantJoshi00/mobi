pub mod handler;
pub mod nodes;
pub mod types;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let machine = {
        #[cfg(feature = "echo")]
        {
            nodes::echo::EchoNode::default()
        }
        #[cfg(feature = "generate")]
        {
            nodes::generate::GenerateNode::default()
        }
        #[cfg(feature = "broadcast-a")]
        {
            nodes::broadcast::BroadcastNode::default()
        }
    };

    handler::executor(machine)
}
