use std::{
    collections::{
        HashMap,
        HashSet,
    }, path::{Path, PathBuf}, sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, Stdin, Stdout}, sync::{RwLock, mpsc::{Receiver, Sender}}
};
use crate::GResult;

type Name = Arc<str>;
type Tag = Name;
type TagPath = Arc<Path>;

pub struct GlasHaus {
    config_path: Arc<Path>,
    pub known_files: HashMap<Name, TagPath>,
    pub        tags: HashMap<Tag, HashSet<Name>>,
}

impl GlasHaus {
    pub fn new(config_path: Arc<Path>) -> Self {
        Self {
            config_path,
            known_files: HashMap::new(),
            tags:        HashMap::new(),
        }
    }

    pub async fn start(self, receiver: Receiver<PathBuf>) -> () {
        let config_path = self.config_path.clone();
        let wrapped_self = RwLock::new(self);
        tokio::spawn(async move {
            Parser::new(receiver, wrapped_self, config_path.clone()).start().await;
        });
        loop {}
    }
}

struct Parser {
    receiver: Receiver<PathBuf>,
    runtime: RwLock<GlasHaus>,
    config_path: Arc<Path>,
}
impl Parser {
    pub fn new(
        receiver: Receiver<PathBuf>,
        runtime: RwLock<GlasHaus>,
        config_path: Arc<Path>,
    ) -> Self {
        Self {
            receiver,
            runtime,
            config_path,
        }
    }
    pub async fn start(mut self) -> () {
        {
            let map = self.parse_tag_file("").await.expect("Currently cant really error out.");
            let mut glashaus = self.runtime.write().await;
            glashaus.tags = map;
        }
        while let Some(file) = self.receiver.recv().await {
            _ = self.parse_md(file);
        }
        todo!()
    }
    async fn parse_tag_file(&self, path: impl AsRef<Path>) -> GResult<HashMap<Tag, HashSet<Name>>> {
        let mut file;
        let mut source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
        }
        else {file = tokio::fs::File::open(path).await?;}
        let _ = file.read_to_string(&mut source).await?;

        let lines = source.lines();
        let mut tags = HashMap::new();
        let mut tagged_names = HashSet::new();
        let mut tag = Arc::from("");
        let mut designator_char;
        for i in lines {
            designator_char = i.chars().next().unwrap_or_else(|| ' ');
            _ = match designator_char {
                '#' => {
                    _ = tags.insert(tag, tagged_names.clone());
                    _ = tagged_names.clear();
                    tag = Arc::from(&i[1..]);
                },
                ';' => _ = tagged_names.insert(Arc::from((&i[1..]).as_ref())),
                _ => continue,
            };
        }
        Ok(tags)
    }
    async fn parse_md(&self, _file: PathBuf) -> GResult<()> {
        todo!()
    }
}

pub enum GLAPICommand {
    QueryTag(String),
    QueryTags(Vec<String>),
    GetNameOrAlias(String),
    None
}

pub enum GLAPIResponse {
    QueryResponse(Vec<(Name, TagPath)>),
    Name(Name),
    None
}

pub struct GlasSocket {
    stdin: Stdin,
    stdout: Stdout,
}

impl GlasSocket {
    pub fn new() -> Self { todo!("Implement new for GlasSocket") }
    pub async fn response(&mut self, response: GLAPIResponse) -> () {
        todo!("Implement response for GlasSocket")
    }
    pub async fn receive(&mut self) -> GLAPICommand {
        todo!("Implement receive for GlasSocket")
    }
}
