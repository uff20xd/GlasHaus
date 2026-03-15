mod filepoler;
mod server;

use filepoler::*;
use server::{
    GlasHaus,
    Config,
};
use std::sync::LazyLock;
use clap::{
    Parser,
    Subcommand,
};
use tokio::{
    sync::mpsc,
};

pub type GResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_file());

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: CliArgs,
}

#[derive(Subcommand, Debug, Clone)]
enum CliArgs {
    Start {
        name: String
    },
    Init,
    Config {
        setting: String,
        new_value: String
    },
    Test,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        CliArgs::Start { name: _ } => { todo!( ) },
        CliArgs::Init  => { todo!( ) },
        CliArgs::Config { setting: _, new_value: _ } => { todo!( ) },
        CliArgs::Test => { 
            let (sender, receiver) = mpsc::channel(500);
            tokio::spawn(async move {
                Poller::new("./tests/", sender, &*CONFIG).start().await;
            });
            tokio::spawn(async move {
                GlasHaus::new(&*CONFIG).start(receiver).await;
            });
            loop {}
        },
    }
}
