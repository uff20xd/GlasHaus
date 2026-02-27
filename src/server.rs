use std::{
    collections::{
        HashMap,
        HashSet,
    },
    rc::Rc,
    path::{PathBuf, Path},
};
use tokio::{
    sync::RwLock,
    sync::mpsc::Receiver
};
use crate::GResult;

type Name = Rc<str>;
type Tag = Name;

pub struct GlasHaus {
    known_files: HashMap<Name, PathBuf>,
           tags: HashMap<Tag, HashSet<Name>>,
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
    pub async fn parse_tag_file(&self, path: &Path) -> GResult<HashMap<Tag, HashSet<Name>>> {
        let mut file;
        if !path.exists()  {

        }
        else if !path.is_file() {
        }
        else {file = std::fs::File::open(path);}
        todo!()
    }

    pub async fn parse_md(&self, file: PathBuf) -> GResult<()> {
        todo!()
    }
    pub async fn parser(mut self, runtime: RwLock<GlasHaus>, mut receiver: Receiver<PathBuf>) -> () {
        while let Some(file) = receiver.recv().await {
            self.parse_md(file);
        }
        todo!()
    }
}
