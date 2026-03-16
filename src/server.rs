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
    pub fn new(config: &'static Config, names: HashMap<Name, TagPath>, tags: HashMap<Tag, HashSet<Name>>) -> Self {
        Self {
            config,
            names,
            tags,
        }
    }

    pub async fn start(self, receiver: Receiver<PathBuf>) -> () {
        self.setup_haus_dir().await;
        let config = self.config;
        let wrapped_self = Arc::new(RwLock::new(self));
        // let mut io_socket = GlasSocket::new();
        println!("From GlasHaus::start");
        join!(
            GlasParser::new(receiver, wrapped_self.clone(), config).start(),
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
    pub fn append_tags(&mut self, tags: Vec<Tag>, names: HashSet<Name>) { 
        for tag in tags.into_iter() {
            if self.tags.contains_key(&*tag) {
                let mut found_names = self.tags.get_mut(&*tag)
                    .expect("This Tag exists, because its checked by contains_key()");
                found_names.extend(names.clone().into_iter());
            } else {
                self.tags.insert(tag, names.clone());
            }
        }
    }
    pub fn append_names(&mut self, names: HashSet<Name>, path: TagPath) {
        for name in names.into_iter() {
            self.names.insert(name, path.clone());
        }
    }
}

pub struct GlasParser {
    receiver: Receiver<PathBuf>,
    runtime: Arc<RwLock<GlasHaus>>,
    config: &'static Config,
}
impl GlasParser {
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
        println!("From GlasParser::start");
        let mut name_file_path = self.config.haus_path.clone();
        name_file_path.push("name_file");
        let mut tag_file_path = self.config.haus_path.clone();
        tag_file_path.push("tag_file");

        // Maybe make this into a feature, but pcs are fast nowadays so startup I can stomach
        // {
        //     let tags = self.parse_tag_file(tag_file_path).await;
        //     let names = self.parse_name_file(name_file_path).await;
        //     let mut glashaus = self.runtime.write().await;
        //     glashaus.tags = tags;
        //     glashaus.names = names;
        // }
        while let Some(file) = self.receiver.recv().await {
            println!("Parsing File: {}", file.display());
            _ = self.parse_md(file).await;
        }
    }
    pub async fn parse_tag_file(path: impl AsRef<Path>) -> HashMap<Tag, HashSet<Name>> {
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
        for line in lines {
            designator_char = line.chars().next().unwrap_or_else(|| ' ');
            _ = match designator_char {
                '#' => {
                    tag = Arc::from(&line[1..]);
                    _ = tags.insert(tag, tagged_names.clone());
                    _ = tagged_names.clear();
                },
                ';' => _ = tagged_names.insert(Arc::from((&line[1..]).as_ref())),
                _ => continue,
            };
        }
        tags
    }
    pub async fn parse_name_file(path: impl AsRef<Path>) -> HashMap<Name, TagPath> {
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
        for line in lines {
            designator_char = line.chars().next().unwrap_or_else(|| ' ');
            _ = match designator_char {
                '=' => {
                    names_to_path.insert(current_name.clone(), Arc::from((&line[1..]).as_ref()));
                },
                ';' => {
                    current_name = Arc::from((&line[1..]).as_ref());
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

        let this_path: TagPath = path.into();
        let this_name: Name = this_path.file_stem().expect("File doesnt have a name for some reason.").to_string_lossy().into();
        let mut sections: HashMap<String, usize> = HashMap::new();
        let mut tags: Vec<Tag> = Vec::new();
        let mut names: HashSet<Name> = HashSet::new();
        names.insert(this_name.clone());
        let mut key_buf = String::new();
        let mut directive_found = false;
        let mut escmode = false;
        let mut string_buf = String::new();
        for (index, line) in source.lines().enumerate() {
            string_buf.clear();
            key_buf.clear();
            escmode = false;
            directive_found = false;
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
                                tags.push(Arc::from(string_buf.clone()));
                            }
                            string_buf.clear();
                            key_buf.clear();
                            break
                        }
                        else if character == ' ' {
                            if string_buf.len() != 0 {
                                tags.push(Arc::from(string_buf.clone()));
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
                                names.insert(Arc::from(string_buf.clone()));
                            }
                            continue;
                        }
                        else if character == ';' || character == '\n' {
                            if string_buf.len() != 0 {
                                names.insert(Arc::from(string_buf.clone()));
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
            write_lock.append_names(names.clone(), this_path);
            write_lock.append_tags(tags, names);
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
            let _sleep = sleep(Duration::from_millis(400)).await;
            self.compile_name_file(&name_file_path).await.expect("Fix it when it happens!");
            self.compile_tag_file(&tag_file_path).await.expect("Fix it when it happens!");
        }
    }
    async fn compile_name_file(&self, path: impl AsRef<Path>) -> GResult<()>  {
        let mut file;
        let _source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
            println!("Namefile doesnt exist, create new")
        }
        else {file = tokio::fs::File::options().write(true).open(path).await?;}
        let read_lock = self.runtime.read().await;
        let names = read_lock.names.clone();
        drop(read_lock);
        let mut buf = String::new();
        for (name, path) in names.iter() {
            buf = buf + ";" + name + "\n" + 
                "=" + &AsRef::<std::ffi::OsStr>::as_ref(&(**path)).to_string_lossy() + "\n";
        }
        file.write(buf.as_bytes()).await?;
        Ok(())
    }
    async fn compile_tag_file(&self, path: impl AsRef<Path>) -> GResult<()>  {
        let mut file;
        let _source = String::new();
        let path = path.as_ref();
        if !path.exists()  {
            file = tokio::fs::File::create_new(path).await?;
            println!("Tagfile doesnt exist, create new")
        }
        else {file = tokio::fs::File::options().write(true).open(path).await?;}
        let read_lock = self.runtime.read().await;
        let tags = read_lock.tags.clone();
        drop(read_lock);

        let mut buf = String::new();
        for (tag, names) in tags.iter() {
            for name in names.iter() {
                buf = buf + ";" + name + "\n";
            }
            buf = buf + "#" + tag + "\n";
        }
        file.write(buf.as_bytes()).await?;
        Ok(())
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
