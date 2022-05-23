use anyhow::Result;
use api::Client;
use appdata::{load_account_info, setup};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use subcmd::{
    download::download,
    list::{list, Sort},
};

mod api;
mod appdata;
mod subcmd;

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
    #[clap(alias = "ls", alias = "l")]
    List {
        path: PathBuf,
        #[clap(short, long, default_value = "fsshare")]
        volume_name: String,
        #[clap(long, default_value = "none", name = "sort-field")]
        sort: Sort,
        #[clap(long, name = "name-only")]
        name_only: bool,
    },
    #[clap(arg_required_else_help = true)]
    #[clap(alias = "dl", alias = "d")]
    Download {
        pathes: Vec<PathBuf>,
        #[clap(short, long, default_value = "fsshare")]
        volume_name: String,
        #[clap(short, long, default_value = ".")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let (username, password);
    let args = match Cli::try_parse() {
        Ok(args) => {
            (username, password) = match load_account_info() {
                Ok(info) => info,
                Err(_) => setup().await?,
            };
            args
        }
        Err(e) => e.exit(),
    };

    let client = Client::with_token_or_login(&username, &password).await?;
    match args.command {
        Command::List {
            path,
            volume_name,
            sort,
            name_only,
        } => list(client, path, &volume_name, sort, name_only).await?,
        Command::Download {
            pathes,
            volume_name,
            output,
        } => download(client, pathes, &volume_name, output).await?,
    }

    Ok(())
}
