mod filepoler;
mod server;

use filepoler::*;
use server::{
    GlasHaus,
    Config,
    GlasParser,
    PipeManager,
};
use std::sync::LazyLock;
use std::sync::Arc;
use std::collections::HashMap;
use std::os::fd::AsFd;
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
    Query {
        tags: Vec<String>,
    },
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
                GlasHaus::new(&*CONFIG, HashMap::new(), HashMap::new()).start(receiver),
            );
        },
        CliArgs::Query { tags } => {
            let mut name_file_path = CONFIG.haus_path.clone();
            name_file_path.push("name_file");
            let mut tag_file_path = CONFIG.haus_path.clone();
            tag_file_path.push("tag_file");

            let names = GlasParser::parse_name_file(name_file_path).await;
            let native_tags = GlasParser::parse_tag_file(tag_file_path).await;
            dbg!(&names);
            dbg!(&native_tags);
            let glashaus = GlasHaus::new(&*CONFIG, names, native_tags);
            let query = glashaus.query_tags(true, " >", tags.into_iter().map(|item| Arc::from(item)).collect());
            println!("From Query: \n{}", query);
        },
        CliArgs::Init  => { todo!( ) },
        CliArgs::Config { setting: _, new_value: _ } => { todo!( ) },
        CliArgs::Test => { 
            let pipes = PipeManager::new().expect("Please give me da pipe.");
            loop {
                // println!("pipe_in: {:?}", fd);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        },
    }
}
