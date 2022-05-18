use anyhow::Result;
use api::{Client, VolumeID};
use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

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
    List {
        path: PathBuf,
        #[clap(short, default_value = "fsshare")]
        volume_name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path(".env")?;
    let args = Cli::parse();

    let client = Client::with_token_or_login(
        env::var("CVPN_USERNAME")?.as_str(),
        env::var("CVPN_PASSWORD")?.as_str(),
    )
    .await?;

    match args.command {
        Command::List { path, volume_name } => {
            let volume_id = VolumeID::from_str(&volume_name)?;
            let segments = client.list(path, &volume_id).await?;
            for segment in segments {
                println!("{}", segment.name);
            }
        }
    }

    Ok(())
}
