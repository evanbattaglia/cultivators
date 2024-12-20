use clap::Parser;

use cultivators::{app, cli, cli::Args, env::init_env};

pub fn main() -> color_eyre::Result<()> {
    init_env();
    let args = Args::parse();
    match cli::app_command(&args) {
        Some(cli::App { tls }) => {
            if *tls {
                cultivators::env::override_enable_tls();
            }
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                app::run().await;
            });
        }
        None => {
            color_eyre::install()?;
            cli::run_cli_command(args)?;
        }
    }
    Ok(())
}
