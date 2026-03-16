use std::{
    collections::{
        HashMap,
        HashSet,
    }, path::{Path, PathBuf}, 
    sync::Arc, 
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{RwLock, mpsc::Receiver},
    join,
    task::yield_now,
    time::{Duration, sleep},
};
use crate::GResult;

type Name = Arc<str>;
type Tag = Name;
type TagPath = Arc<Path>;

pub struct GlasHaus {
    config: &'static Config,
    pub names: HashMap<Name, TagPath>,
    pub  tags: HashMap<Tag, HashSet<Name>>,
}

impl GlasHaus {
    pub fn new(config: &'static Config) -> Self {
        Self {
            config,
            names: HashMap::new(),
            tags:  HashMap::new(),
        }
    }

    pub async fn start(self, receiver: Receiver<PathBuf>) -> () {
        self.setup_haus_dir().await;
        let config = self.config;
        let wrapped_self = Arc::new(RwLock::new(self));
        // let mut io_socket = GlasSocket::new();
        println!("From GlasHaus::start");
        join!(
            Parser::new(receiver, wrapped_self.clone(), config).start(),
            GlasWriter::new(wrapped_self.clone(), config).start(),
        );
    }
    pub async fn setup_haus_dir(&self) -> () {
        let _source = String::new();
        let path: &Path = self.config.haus_path.as_ref();
        tokio::fs::DirBuilder::new()
            .recursive(true)
            .create(path)
            .await
            .expect("It doesnt exist so i should be able to create it.");
    }
    pub fn query_tags(&self, tags: Vec<Tag>) -> String {
        let elseh = HashSet::new(); let names = self.tags.get(&tags[0]).unwrap_or_else(|| &elseh);
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
    pub fn append_tags(&mut self, tags: HashMap<Tag, HashSet<Name>>) { 
        for (tag, names) in tags.into_iter() {
            if self.tags.contains_key(&*tag) {
                let mut found_names = self.tags.get_mut(&*tag)
                    .expect("This Tag exists, because its checked by contains_key()");
                found_names.extend(names.into_iter());
            } else {
                self.tags.insert(tag, names);
            }
        }
        todo!("Implement GlasHaus::append_tags()");
    }
}

struct Parser {
    receiver: Receiver<PathBuf>,
    runtime: Arc<RwLock<GlasHaus>>,
    config: &'static Config,
}
impl Parser {
    pub fn new(
        receiver: Receiver<PathBuf>,
        runtime: Arc<RwLock<GlasHaus>>,
        config: &'static Config,
    ) -> Self {
        Self {
            receiver,
            runtime,
            config,
        }
    }
    pub async fn start(mut self) -> () {
        println!("From Parser::start");
        let mut name_file_path = self.config.haus_path.clone();
        name_file_path.push("name_file");
        let mut tag_file_path = self.config.haus_path.clone();
        tag_file_path.push("tag_file");
        {
            let tags = self.parse_tag_file(tag_file_path).await;
            let names = self.parse_name_file(name_file_path).await;
            let mut glashaus = self.runtime.write().await;
            glashaus.tags = tags;
            glashaus.names = names;
        }
        while let Some(file) = self.receiver.recv().await {
            println!("Parsing File: {}", file.display());
            _ = self.parse_md(file).await;
        }
    }
    async fn parse_tag_file(&self, path: impl AsRef<Path>) -> HashMap<Tag, HashSet<Name>> {
        let mut file;
        let mut source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await.expect("It doesnt exist so i should be able to create it.");
        }
        else {file = tokio::fs::File::open(path).await.expect("The Path should exist. It could be a directory in which case go fuck yourself.");}
        let _ = file.read_to_string(&mut source).await.expect("This File should be readable as checked above");

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
        tags
    }
    async fn parse_name_file(&self, path: impl AsRef<Path>) -> HashMap<Name, TagPath> {
        let mut file;
        let mut source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await.expect("The path doesnt exist and it should be created here.");
        }
        else {file = tokio::fs::File::open(path).await.expect("The Path should exist. It could be a directory in which case go fuck yourself.");}
        let _ = file.read_to_string(&mut source).await.expect("Why shouldnt this be readable. Tell me.");

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
        names_to_path
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
        let this_name: Name = path_to_self.file_stem().expect("File doesnt have a name for some reason.").to_string_lossy().into();
        let mut sections: HashMap<String, usize> = HashMap::new();
        let mut tags: HashMap<Tag, Name> = HashMap::new();
        let mut names: HashMap<Name, TagPath> = HashMap::new();
        names.insert(this_name.clone(), path_to_self.clone());
        let mut key_buf = String::new();
        let mut directive_found = false;
        let mut escmode = false;
        let mut string_buf = String::new();
        for (index, line) in source.lines().enumerate() {
            if line.chars().next().unwrap_or_else(|| ' ') == '#' {
                sections.insert(line[1..].trim_ascii().to_owned(), index);
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
                        else if character == ';' || character == '\n' {
                            if string_buf.len() != 0 {
                                tags.insert(Arc::from(string_buf.clone()), this_name.clone());
                            }
                            string_buf.clear();
                            key_buf.clear();
                            break
                        }
                        else if character == ' ' {
                            if string_buf.len() != 0 {
                                tags.insert(Arc::from(string_buf.clone()), this_name.clone());
                            }
                            continue;
                        }
                        else {
                            string_buf.push(character);
                            continue;
                        }
                    }
                    else if key_buf == "alias" {
                        if escmode {
                            string_buf.push(character);
                            continue;
                        }
                        else if character == '\\' {
                            escmode = true;
                            continue;
                        }
                        else if character == ' ' {
                            if string_buf.len() != 0 {
                                names.insert(Arc::from(string_buf.clone()), path_to_self.clone());
                            }
                            continue;
                        }
                        else if character == ';' || character == '\n' {
                            if string_buf.len() != 0 {
                                names.insert(Arc::from(string_buf.clone()), path_to_self.clone());
                            }
                            string_buf.clear();
                            key_buf.clear();
                            break
                        }
                        else {
                            string_buf.push(character);
                            continue;
                        }
                    }
                    else {
                        key_buf.push(character);
                    }
                }
            }
        }
        dbg!(&sections);
        dbg!(&names);
        dbg!(&tags);
         {
             let mut write_lock = self.runtime.write().await;
             write_lock.names.extend(names.into_iter());
         }
        Ok(())
    }
}

pub struct GlasWriter {
    config: &'static Config,
    runtime: Arc<RwLock<GlasHaus>>,
}
impl GlasWriter {
    pub fn new(
        runtime: Arc<RwLock<GlasHaus>>,
        config: &'static Config,
        ) -> Self {
        Self {
            runtime,
            config,
        }
    }
    pub async fn start(mut self) -> Self {
        let mut name_file_path = self.config.haus_path.clone();
        name_file_path.push("name_file");
        let mut tag_file_path = self.config.haus_path.clone();
        tag_file_path.push("tag_file");
        loop {
            let _sleep = sleep(Duration::from_secs(1)).await;
            self.compile_name_file(&name_file_path).await.expect("Fix it when it happens!");
            // self.compile_tag_file(&name_file_path).await.expect("Fix it when it happens!");
        }
    }
    async fn compile_name_file(&self, path: impl AsRef<Path>) -> GResult<()>  {
        println!("Level 0");
        let mut file;
        let _source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
        }
        else {file = tokio::fs::File::open(path).await?;}
        println!("Level 1");
        let read_lock = self.runtime.read().await;
        println!("Level 2");
        let names = read_lock.names.clone();
        drop(read_lock);
        println!("Level 3");
        let mut buf = String::new();
        for (name, path) in names.iter() {
            buf = buf + name + "\n" + 
                &AsRef::<std::ffi::OsStr>::as_ref(&(**path)).to_string_lossy() + "\n";
            println!("Level 3.1");
        }
        println!("Level 4");
        file.write(buf.as_bytes()).await?;
        println!("Level 5");
        Ok(())
    }
    async fn compile_tag_file() -> GResult<()>  {
        todo!()
    }
}

pub struct Config {
    pub haus_path: PathBuf,
}

impl Config {
    pub fn from_file() -> Self { 
        Self {
            haus_path: "./.glashaus/".into(),
        }
    }
}
