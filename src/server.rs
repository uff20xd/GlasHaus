use std::{
    collections::{
        HashMap,
        HashSet,
    }, io::Read, path::{Path, PathBuf}, rc::Rc
};
use tokio::{
    sync::RwLock,
    sync::mpsc::Receiver,
    io::AsyncReadExt
};
use crate::GResult;

type Name = Rc<str>;
type Tag = Name;

pub struct GlasHaus {
    pub known_files: HashMap<Name, PathBuf>,
    pub        tags: HashMap<Tag, HashSet<Name>>,
}

impl GlasHaus {
    pub fn new() -> Self {
        Self {
            known_files: HashMap::new(),
            tags:        HashMap::new(),
        }
    }

    pub async fn start(mut self) -> () {
        let wrapped_self = RwLock::new(self);
        todo!()
    }
}

struct Parser {
    receiver: Receiver<PathBuf>,
    runtime: RwLock<GlasHaus>,
}
impl Parser {
    pub fn new(
        receiver: Receiver<PathBuf>,
        runtime: RwLock<GlasHaus>
    ) -> Self {
        Self {
            receiver,
            runtime,
        }
    }
    pub async fn parse_tag_file(&self, path: impl AsRef<Path>) -> GResult<HashMap<Tag, HashSet<Name>>> {
        let mut file;
        let mut source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
        }
        else {file = tokio::fs::File::open(path).await?;}
        let _ = file.read_to_string(&mut source).await?;
        let mut lines = source.lines();
        todo!()
    }

    pub async fn parse_md(&self, file: PathBuf) -> GResult<()> {
        todo!()
    }
    pub async fn parser(mut self, runtime: RwLock<GlasHaus>, mut receiver: Receiver<PathBuf>) -> () {
        {
            let glashaus = runtime.write();
            let map = self.parse_tag_file("");
        }
        while let Some(file) = receiver.recv().await {
            _ = self.parse_md(file);
        }
        todo!()
    }
}
