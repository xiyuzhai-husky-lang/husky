use crate::error::DiskCacheResult;
use crate::{entry::LlmCacheEntry, seed::IsDiskCacheSeed};
use attach::Attach;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::{fs, marker::PhantomData};

pub struct LlmCacheSaveThread<Db, Seed, Request, Response>
where
    Seed: IsDiskCacheSeed,
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    internal: Option<LlmCacheSaveThreadInternal<Db, Seed, Request, Response>>,
}

struct LlmCacheSaveThreadInternal<Db, Seed, Request, Response>
where
    Seed: IsDiskCacheSeed,
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    db: Db,
    sender: Sender<String>,
    thread: std::thread::JoinHandle<()>,
    phantom: PhantomData<(Seed, Request, Response)>,
}

impl<Db, Seed, Request, Response> LlmCacheSaveThread<Db, Seed, Request, Response>
where
    Db: Attach,
    Seed: IsDiskCacheSeed,
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    pub fn new(
        db: Db,
        path: PathBuf,
        mut entries: Vec<LlmCacheEntry<Seed, Request, Response>>,
    ) -> Self {
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

    pub fn save(&self, entries: &[LlmCacheEntry<Seed, Request, Response>]) -> DiskCacheResult<()> {
        self.internal.as_ref().unwrap().save(entries)
    }
}

impl<Db, Seed, Request, Response> LlmCacheSaveThreadInternal<Db, Seed, Request, Response>
where
    Db: Attach,
    Seed: IsDiskCacheSeed,
    Request: Serialize + DeserializeOwned + Eq + std::hash::Hash + Clone + Send + 'static,
    Response: Serialize + DeserializeOwned + Clone + Send + 'static,
{
    fn save(&self, entries: &[LlmCacheEntry<Seed, Request, Response>]) -> DiskCacheResult<()> {
        self.sender
            .send(
                self.db
                    .attach(|| serde_json::to_string_pretty(entries).unwrap()),
            )
            .map_err(|_| todo!())?;
        Ok(())
    }
}

impl<Db, Seed, Request, Response> Drop for LlmCacheSaveThread<Db, Seed, Request, Response>
where
    Seed: IsDiskCacheSeed,
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
