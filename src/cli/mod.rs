mod args;
mod gen_commands;
mod jwt_commands;
mod reg_commands;
mod req_commands;
mod util;

pub use args::{App, Args};

use args::Commands;
use color_eyre::eyre;

pub fn app_command(args: &Args) -> Option<&args::App> {
    match &args.command {
        Commands::App(app) => Some(app),
        _ => None,
    }
}

pub fn run_cli_command(args: Args) -> eyre::Result<()> {
    use Commands::*;
    match args.command {
        App(_) => unreachable!("Should be handled in bin.rs"),
        Reg { subcommand } => reg_commands::main(subcommand)?,
        Req { subcommand } => req_commands::main(subcommand)?,
        Gen { subcommand } => gen_commands::main(subcommand)?,
        Jwt { subcommand } => jwt_commands::main(subcommand)?,
        ShowDefaultEnv => crate::env::print_default_env(),
    };
    Ok(())
}
