use std::{
    path::{PathBuf, Path},
    time::SystemTime,
};
use tokio::{
    time::{
        sleep,
        Duration,
    },
    sync::mpsc::Sender,
};
use crate::GResult;
use crate::server::Config;

pub struct Poller {
    path: PathBuf,
    channel: Sender<PathBuf>,
    change_date: SystemTime,
    new_change_date: SystemTime,
    config: &'static Config,
}

impl Poller {
    pub fn new(path: impl AsRef<Path>, sender: Sender<PathBuf>, config: &'static Config) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            channel: sender,
            change_date: SystemTime::UNIX_EPOCH,
            new_change_date: SystemTime::UNIX_EPOCH,
            config,
        }
    }
    pub async fn start(mut self) -> bool {
        println!("in poll");
        let _sleep = sleep(Duration::from_millis(50)).await;
        loop {
            let search = self.poll();
            let _ = match search.await {
                Err(_) => {break},
                _ => {}
            };
            self.change_date = self.new_change_date;
            let _sleep = sleep(Duration::from_millis(200)).await;
        }
        false
    }
    pub async fn poll(&mut self) -> GResult<()> {
        let dir = self.path.read_dir().unwrap();
        for i in dir {
            let file = i.unwrap();
            let metadata = file.metadata().unwrap();
            let modified = metadata.modified().unwrap();
            if self.change_date < modified {
                println!("Recognised Modified File: {}", file.path().display());
                _ = self.channel.send(file.path()).await;
                if self.new_change_date < modified {
                    self.new_change_date = modified;
                }
            }
        }
        Ok(())
    }
}
