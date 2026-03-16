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
    join,
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
    Start,
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
        CliArgs::Start => {
            let (sender, receiver) = mpsc::channel(500);
            join!(
                Poller::new("./tests/", sender, &*CONFIG).start(),
                GlasHaus::new(&*CONFIG).start(receiver),
            );
        },
        CliArgs::Init  => { todo!( ) },
        CliArgs::Config { setting: _, new_value: _ } => { todo!( ) },
        CliArgs::Test => { 
            
        },
    }
}
