use std::{
    path::{PathBuf, Path},
    time::SystemTime,
};
use tokio::{
    time::{
        sleep,
        Duration,
    },
};
use crate::error::*;

pub struct Poller {
    path: PathBuf,
    changed_files: Vec<PathBuf>,
    change_date: SystemTime,
    new_change_date: SystemTime,
}

impl Poller {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            changed_files: Vec::new(),
            change_date: SystemTime::UNIX_EPOCH,
            new_change_date: SystemTime::UNIX_EPOCH,
        }
    }
    pub async fn start(mut self) -> bool {
        println!("in poll");
        loop {
            println!("in poll loop");
            let search = self.poll();
            let _sleep = sleep(Duration::from_secs(2)).await;
            let _ = match search.await {
                Err(_) => {break},
                _ => {}
            };
        }
        false
    }
    pub async fn poll(&mut self) -> Result<(), GlassError> {
        let dir = self.path.read_dir().unwrap();
        for i in dir {
            let file = i.unwrap();
            let metadata = file.metadata().unwrap();
            let modified = metadata.modified().unwrap();
            if self.change_date < modified {
                println!("Recognised Modified File: {}", file.path().display());
                self.changed_files.push(file.path());
                if self.new_change_date < modified {
                    self.new_change_date = modified;
                }
            }
        }
        Ok(())
    }
}
