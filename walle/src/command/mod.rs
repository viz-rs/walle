use anyhow::Result;
use clap::Subcommand;

mod client;

mod server;

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum WalleCmd {
    #[command(flatten)]
    Client(client::Cmd),

    /// Start a walle server
    #[command(subcommand)]
    Server(server::Cmd),
}

impl WalleCmd {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Client(client) => client.run(),
            Self::Server(server) => server.run(),
        }
    }
}
