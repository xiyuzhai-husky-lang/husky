use crate::entry::LlmCacheEntry;
use crate::error::LlmCacheResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};

pub struct LlmCacheSaveThread<Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    internal: Option<LlmCacheSaveThreadInternal<Request, Response>>,
}

struct LlmCacheSaveThreadInternal<Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    sender: Sender<LlmCacheEntry<Request, Response>>,
    thread: std::thread::JoinHandle<()>,
}

impl<Request, Response> LlmCacheSaveThread<Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    pub fn new(path: PathBuf, mut entries: Vec<LlmCacheEntry<Request, Response>>) -> Self {
        let (sender, receiver) = channel();

        let thread = std::thread::spawn(move || {
            while let Ok(entry) = receiver.recv() {
                entries.push(entry);

                // Write entire cache to file
                if let Ok(contents) = serde_json::to_string_pretty(&entries) {
                    // Write to temporary file first
                    if let Err(e) = fs::write(&path, contents) {
                        todo!()
                    }
                }
            }
        });

        Self {
            internal: Some(LlmCacheSaveThreadInternal { sender, thread }),
        }
    }

    pub fn save(&self, entry: LlmCacheEntry<Request, Response>) -> LlmCacheResult<()> {
        self.internal.as_ref().unwrap().save(entry)
    }
}

impl<Request, Response> LlmCacheSaveThreadInternal<Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    fn save(&self, entry: LlmCacheEntry<Request, Response>) -> LlmCacheResult<()> {
        self.sender.send(entry).map_err(|_| todo!())?;
        Ok(())
    }
}

impl<Request, Response> Drop for LlmCacheSaveThread<Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    fn drop(&mut self) {
        let Some(LlmCacheSaveThreadInternal { sender, thread }) =
            std::mem::take(&mut self.internal)
        else {
            unreachable!()
        };
        drop(sender);
        if let Err(e) = thread.join() {
            eprintln!("Error joining cache save thread: {:?}", e);
        }
    }
}
