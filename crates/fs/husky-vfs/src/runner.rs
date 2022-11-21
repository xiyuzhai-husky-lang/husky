// use crate::*;
// use std::sync::Arc;
// use salsa::{ParallelDatabase, Snapshot};

// #[derive(Default)]
// pub struct VfsDbRunner<DB: VfsDb + ParallelDatabase + 'static>(Arc<Mutex<DB>>);

// impl<DB: VfsDb + ParallelDatabase + 'static> VfsDbRunner<DB> {
//     pub fn new(db: DB) -> Self {
//         Self(Arc::new(Mutex::new(db)))
//     }

//     pub fn launch_watcher(&self) -> VfsWatcherThread<DB>
//     where
//         DB: HasWatcherPlace,
//     {
//         VfsWatcherThread::new(self.0.clone())
//     }

//     pub fn snapshot(&self) -> Snapshot<DB> {
//         self.0.lock().unwrap().snapshot()
//     }
// }
