mod filepoler;
mod server;
mod socket;

use filepoler::*;
use clap::{
    Parser,
    Subcommand,
};
use tokio::{
    sync::mpsc,
};

pub type GResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
            let (sender, mut receiver) = mpsc::channel(100);
            tokio::spawn(async move {
                Poller::new("./tests/", sender).start().await;
            });
            tokio::spawn(async move {
                while let Some(file) = receiver.recv().await {
                    println!("Processing File {}", file.display());
                }
            });
            loop {}
        },
    }
}
