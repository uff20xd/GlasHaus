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
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    NoneWell,
    None,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
