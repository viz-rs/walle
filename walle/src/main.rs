use std::io::{self, Write};

use clap::{crate_authors, crate_description, crate_version, CommandFactory, Parser, Subcommand};
// use clap_complete::{generate, Shell as CompletionShell};

#[derive(Parser, Debug)]
#[clap(
    author=crate_authors!(),
    version=crate_version!(),
    about=crate_description!(),
    subcommand_required=true,
    arg_required_else_help=true,
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a project to tracking
    Add,
    /// Remove a project from tracking list
    Remove,
    /// List all projects being tracked
    List,
}

fn main() {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            let _ = e.print();
            let code = if e.use_stderr() {
                let _ = writeln!(
                    io::stderr(),
                    "\nNOTE:\n    passed arguments: {:?}",
                    // collect into a vec to format args as a slice
                    std::env::args().skip(1).collect::<Vec<_>>()
                );
                2
            } else {
                0
            };
            std::process::exit(code);
        }
    };

    match args.command {
        Commands::Add => {}
        Commands::Remove => {}
        Commands::List => {}
    }
}
