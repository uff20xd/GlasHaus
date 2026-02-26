use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
    rc::Rc,
    path::PathBuf,
};
use tokio::sync::mpsc::Receiver;

type Name = Rc<str>;
type Tag = Name;

pub struct GlasHaus {
    receiver: Receiver<PathBuf>,
    known_files: HashMap<Name, PathBuf>,
           tags: HashMap<Tag, HashSet<Name>>,
}

impl GlasHaus {
    pub fn new(receiver: Receiver<PathBuf>) -> Self {
        Self {
            receiver,
            known_files: HashMap::new(),
            tags:        HashMap::new(),
        }
    }
}
