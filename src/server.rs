use std::{
    collections::{
        HashMap,
        HashSet,
    }, path::{Path, PathBuf}, sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, sync::{RwLock, mpsc::Receiver}
};
use crate::GResult;

type Name = Arc<str>;
type Tag = Name;
type TagPath = Arc<Path>;

pub struct GlasHaus {
    config_path: Arc<Path>,
    pub names: HashMap<Name, TagPath>,
    pub  tags: HashMap<Tag, HashSet<Name>>,
}

impl GlasHaus {
    pub fn new(config_path: Arc<Path>) -> Self {
        Self {
            config_path,
            names: HashMap::new(),
            tags:        HashMap::new(),
        }
    }

    pub async fn start(self, receiver: Receiver<PathBuf>) -> () {
        let config_path = self.config_path.clone();
        let wrapped_self = RwLock::new(self);
        // let mut io_socket = GlasSocket::new();
        tokio::spawn(async move {
            Parser::new(receiver, wrapped_self, config_path.clone()).start().await;
        });
        loop {

        }
    }
    pub fn query_tags(&self, tags: Vec<Tag>) -> String {
        let elseh = HashSet::new();
        let mut names = self.tags.get(&tags[0]).unwrap_or_else(|| &elseh);
        let mut query: std::collections::LinkedList<&Name> = names
            .iter()
            .filter(|name| 
                !tags
                .iter()
                .any(|tag| tag.contains(&(***name)))
                )
            .collect();
        let mut ret: String;
        if let Some(name) = query.pop_front() {
            ret = (**name).into();
        } else {
            ret = String::new();
        }
        for name in query {
            ret = ret + "\n" + name.as_ref();
        }
        ret
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
    async fn parse_name_file(&self, path: impl AsRef<Path>) -> GResult<HashMap<Name, TagPath>> {
        let mut file;
        let mut source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
        }
        else {file = tokio::fs::File::open(path).await?;}
        let _ = file.read_to_string(&mut source).await?;

        let lines = source.lines();
        let mut names_to_path = HashMap::new();
        let mut current_name: Name = Arc::from("");
        let mut designator_char;
        for i in lines {
            designator_char = i.chars().next().unwrap_or_else(|| ' ');
            _ = match designator_char {
                '=' => {
                    names_to_path.insert(current_name.clone(), Arc::from((&i[1..]).as_ref()));
                },
                ';' => {
                    current_name = Arc::from((&i[1..]).as_ref());
                },
                _ => continue,
            };
        }
        Ok(names_to_path)
    }
    async fn compile_name_file(&self, path: impl AsRef<Path>) -> GResult<()>  {
        let mut file;
        let mut source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
        }
        else {file = tokio::fs::File::open(path).await?;}
        let read_lock = self.runtime.read().await;
        let mut names = read_lock.names.clone();
        drop(read_lock);
        let mut buf = String::new();
        for (name, path) in names.iter() {
            buf = buf + name + "\n" + 
                &AsRef::<std::ffi::OsStr>::as_ref(&(**path)).to_string_lossy() + "\n";
        }
        file.write(buf.as_bytes()).await?;
        Ok(())
    }
    async fn compile_tag_file() -> GResult<()>  {
        todo!()
    }
    async fn parse_md(&mut self, path: PathBuf) -> GResult<()> {
        let mut file;
        let mut source = String::new();
        if !path.exists()  {
            file = tokio::fs::File::create_new(&path).await?;
        }
        else {file = tokio::fs::File::open(&path).await?;}
        _ = file.read_to_string(&mut source).await;

        let path_to_self: TagPath = path.into();
        let this_name = path_to_self.file_stem().unwrap_or_else(|| "".into())
        let mut sections: HashMap<String, usize> = HashMap::new();
        let mut tags: HashMap<Tag, Name> = HashMap::new();
        let mut names: HashMap<Name, TagPath> = HashMap::new();
        let mut key_buf = String::new();
        let mut directive_found = false;
        let mut escmode = false;
        let mut string_buf = String::new();
        for (index, line) in source.lines().enumerate() {
            if line.chars().next().unwrap_or_else(|| ' ') == '#' {
                sections.insert(line[1..0].trim_ascii().to_owned(), index);
                continue
            }
            for character in line.chars() {
                if !directive_found {
                    if character == '@' {
                        directive_found = true;
                    }
                } else {
                    if key_buf == "tags" {
                        if escmode {
                            string_buf.push(character);
                            continue;
                        }
                        else if character == '\\' {
                            escmode = true;
                            continue;
                        }
                        else if character == ' ' {
                            tags.insert()
                            continue;
                        }
                    }
                    else if key_buf == "alias" {
                    }
                    else if key_buf == "["{
                    }
                    else if key_buf == "pic" {
                    }
                    else {
                        key_buf.push(character);
                    }
                }
            }
        }
        Ok(())
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
