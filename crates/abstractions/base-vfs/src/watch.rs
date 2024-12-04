use crate::*;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use eyre::Result;
use notify_debouncer_mini::{
    new_debouncer, notify::RecommendedWatcher, DebounceEventResult, DebouncedEvent, Debouncer,
};
use salsa::{
    snapshot::{Snapshot, SnapshotClone},
    Db, Durability,
};
use sealed::sealed;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, Weak},
    thread,
    time::Duration,
};

#[sealed]
pub trait WatchableVfsDb:
    std::ops::Deref<Target = Db> + std::ops::DerefMut + SnapshotClone + Send + 'static
{
    fn watcher(&self) -> Option<&VfsWatcher>;
}

#[sealed]
impl<T> WatchableVfsDb for T
where
    T: std::ops::Deref<Target = Db> + std::ops::DerefMut + SnapshotClone + Send + 'static,
{
    fn watcher(&self) -> Option<&VfsWatcher> {
        self.vfs_jar().cache().watcher()
    }
}

#[derive(Clone)]
pub struct VfsWatcher(pub(crate) Arc<Mutex<Debouncer<RecommendedWatcher>>>);

impl VfsWatcher {
    fn new(debounce_tx: Sender<DebounceEventResult>) -> Self {
        VfsWatcher(Arc::new(Mutex::new(
            new_debouncer(DEBOUNCE_TIMEOUT, debounce_tx).unwrap(),
        )))
    }
}

const DEBOUNCE_TIMEOUT_RAW: u64 = 50;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(DEBOUNCE_TIMEOUT_RAW);

#[cfg(test)]
pub(crate) const DEBOUNCE_TEST_SLEEP_TIME: Duration =
    Duration::from_millis(DEBOUNCE_TIMEOUT_RAW * 4);

pub struct WatchedVfs<DB: WatchableVfsDb> {
    db: Arc<Mutex<DB>>,
}

impl<DB: WatchableVfsDb> Default for WatchedVfs<DB>
where
    DB: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<DB: WatchableVfsDb> WatchedVfs<DB> {
    pub fn query<S>(&self, f: impl FnOnce(Snapshot<DB>) -> S) -> S {
        f(self.db.lock().unwrap().snapshot())
    }
}

pub struct VfsWatcherInstance<DB: WatchableVfsDb> {
    db: Weak<Mutex<DB>>,
    debounce_rx: Receiver<DebounceEventResult>,
}

impl<DB: WatchableVfsDb> WatchedVfs<DB> {
    pub(crate) fn new(mut db: DB) -> Self {
        let (debounce_tx, debounce_rx) = unbounded();
        db.vfs_jar_mut().set_watcher(VfsWatcher::new(debounce_tx));
        let db = Arc::new(Mutex::new(db));
        thread::spawn({
            let db = Arc::downgrade(&db);
            move || match VfsWatcherInstance::new(db, debounce_rx).run() {
                Ok(_) => (),
                Err(_) => todo!(),
            }
        });
        Self { db }
    }
}

impl<DB: WatchableVfsDb> VfsWatcherInstance<DB> {
    fn new(db: Weak<Mutex<DB>>, debounce_rx: Receiver<DebounceEventResult>) -> Self {
        Self { db, debounce_rx }
    }

    fn run(mut self) -> Result<()> {
        loop {
            select! {
                recv(self.debounce_rx) -> debounce => match debounce {
                        Ok(Ok(debounced_events)) => {
                            let Some(db) = self.db.upgrade() else{
                                break Ok(())
                            };
                            Self::process_debounced_events(db, debounced_events)
                        },
                        Ok(Err(_)) => todo!(),
                        Err(_) => todo!(),
                    },
            }
        }
    }

    fn process_debounced_events(db: Arc<Mutex<DB>>, events: Vec<DebouncedEvent>) {
        for event in events {
            db.lock()
                .unwrap()
                .refresh_file_from_disk(&event.path, /* ad hoc */ Durability::LOW)
                .unwrap()
        }
    }
}

// #[cfg(target_os = "linux")]
#[test]
fn watcher_works() {
    let db = DB::default();
    let tempdir = tempfile::tempdir().unwrap();
    let some_pkg_dir = tempdir.path().join("somepath");
    std::fs::create_dir(&some_pkg_dir).unwrap();
    let path = some_pkg_dir.join("Corgi.toml");
    let abs_path: VirtualPath = VirtualPath::try_new(&db, &path).unwrap();
    let db = WatchedVfs::new(db);

    std::fs::write(&path, "Hello, world!").expect("can't write");
    assert!(db.query(|db| db
        .file_from_virtual_path(abs_path, Durability::LOW)
        .unwrap()
        .content(&db)
        == &FileContent::OnDisk("Hello, world!".to_owned())),);
    std::fs::write(&path, "Hello, world!2").expect("can't write");
    let _a = DEBOUNCE_TEST_SLEEP_TIME;
    std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
    assert!(db.query(|db| db
        .file_from_virtual_path(abs_path, Durability::LOW)
        .unwrap()
        .content(&db)
        == &FileContent::OnDisk("Hello, world!2".to_owned())))
}
