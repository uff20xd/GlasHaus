use tokio::{
    io::{AsyncReadExt, Stdin, Stdout},
    sync::{RwLock, mpsc::{Receiver, Sender}}, 
};

pub enum GLAPICommand {
    QueryTag(String),
    QueryTags(Vec<String>),
    GetNameOrAlias(String),
}

pub struct GlasSocket {
    sender: Sender<GLAPICommand>,
    stdio: (),
}

impl GlasSocket {
    pub fn new() -> (Self, Receiver<GLAPICommand>) { todo!() }
    pub fn start() -> () {
        todo!()
    }
    fn poll(&mut self) -> () {
        todo!()
    }
}
