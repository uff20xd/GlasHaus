mod filepoler;
mod gmd_parser;
mod server;

use clap::{
    Parser,
    Subcommand,
};
use tokio::{
    task,
    sync::*,
};

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
        CliArgs::Start { name } => { todo!( ) },
        CliArgs::Init  => { todo!( ) },
        CliArgs::Config { setting, new_value } => { todo!( ) },
        CliArgs::Test => { todo!( ) },
    }
    println!("Hello, world!");
}
