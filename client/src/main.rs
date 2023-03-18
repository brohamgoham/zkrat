use clap::{Arg, Command};

mod api;
mod cli;
mod config;
mod error;

pub use error::Error;

fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new(cli::AGENTS).about("List all agents!?!?!"))
        .subcommand(Command::new(cli::JOBS).about("YOO LIST ALL JOBBB PLZZZ"))
        .subcommand(
            Command::new(cli::EXEC)
                .about("Your about to EXEC THIS COMMAND SONIC!!! >:0")
                .arg(
                    Arg::new("agent")
                        .short("s")
                        .long("agent")
                        .help("the agent id yo execute the command on!")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::new("command")
                        .help("🦀Command🦀toExecute🦀withARGSZZ"),
                        .required(true)
                        .index(1)
                ),
        )
        arg_required_else_help(true)
        .get_matches();
}
