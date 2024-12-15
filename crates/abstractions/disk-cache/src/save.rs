use crate::entry::LlmCacheEntry;
use crate::error::LlmCacheResult;
use attach::Attach;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::{fs, marker::PhantomData};

pub struct LlmCacheSaveThread<Db, Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    internal: Option<LlmCacheSaveThreadInternal<Db, Request, Response>>,
}

struct LlmCacheSaveThreadInternal<Db, Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    db: Db,
    sender: Sender<String>,
    thread: std::thread::JoinHandle<()>,
    phantom: PhantomData<(Request, Response)>,
}

impl<Db, Request, Response> LlmCacheSaveThread<Db, Request, Response>
where
    Db: Attach,
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    pub fn new(db: Db, path: PathBuf, mut entries: Vec<LlmCacheEntry<Request, Response>>) -> Self {
        let (sender, receiver) = channel();

        let thread = std::thread::spawn(move || {
            while let Ok(content) = receiver.recv() {
                if let Err(e) = fs::write(&path, content) {
                    todo!()
                }
            }
        });

        Self {
            internal: Some(LlmCacheSaveThreadInternal {
                db,
                sender,
                thread,
                phantom: PhantomData,
            }),
        }
    }

    pub fn save(&self, entries: &[LlmCacheEntry<Request, Response>]) -> LlmCacheResult<()> {
        self.internal.as_ref().unwrap().save(entries)
    }
}

impl<Db, Request, Response> LlmCacheSaveThreadInternal<Db, Request, Response>
where
    Db: Attach,
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    fn save(&self, entries: &[LlmCacheEntry<Request, Response>]) -> LlmCacheResult<()> {
        self.sender
            .send(
                self.db
                    .attach(|| serde_json::to_string_pretty(entries).unwrap()),
            )
            .map_err(|_| todo!())?;
        Ok(())
    }
}

impl<Db, Request, Response> Drop for LlmCacheSaveThread<Db, Request, Response>
where
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    fn drop(&mut self) {
        let Some(LlmCacheSaveThreadInternal { sender, thread, .. }) =
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
