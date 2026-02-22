use std::{
    collections::{
        HashMap,
    },
};

type Name = Arc<str>;
type Tag = Arc<str>;

pub struct GlassHaus {
    known_files: HashMap<Name, PathBuf>,
           tags: HashMap<Tag, Vec<Name>>,
}
