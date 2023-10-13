use anyhow::Result;
use clap::Parser;

mod command;

use command::WalleCmd;

#[derive(Parser)]
#[clap(author, about, version)]
struct Walle {
    #[clap(subcommand)]
    walle: WalleCmd,
}

impl Walle {
    fn run(self) -> Result<()> {
        self.walle.run()
    }
}

fn main() -> Result<()> {
    Walle::parse().run()
}
