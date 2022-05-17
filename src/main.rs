use anyhow::Result;
use api::Client;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod api;
mod appdata;

#[derive(Parser, Debug)]
#[clap(name = "cvpn")]
#[clap(about = "A command-line application for Shizuoka University VPN service")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    #[clap(alias = "ls")]
    #[clap(alias = "l")]
    List { path: PathBuf },
}

const USERNAME: &str = "";
const PASSWORD: &str = "";

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let client = Client::with_login(USERNAME, PASSWORD).await?;

    match args.command {
        Command::List { path } => {
            let segments = client.list(path, "fsshare").await?;
            for segment in segments {
                println!("{}", segment.name);
            }
        }
    }

    Ok(())
}
