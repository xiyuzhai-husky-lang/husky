use crate::*;
use crossbeam_channel::select;
use eyre::Result;
use notify_debouncer_mini::{
    new_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
    DebounceEventResult, DebouncedEvent, Debouncer,
};
use salsa::{
    snapshot::{Snapshot, SnapshotClone},
    Db, Durability,
};
use sealed::sealed;
use std::{
    marker::PhantomData,
    sync::{mpsc::Receiver, Arc, Mutex, Weak},
    thread,
    time::Duration,
};

#[sealed]
pub trait WatchableVfsDb:
    std::ops::Deref<Target = Db> + std::ops::DerefMut + SnapshotClone + Send + 'static
{
}

#[sealed]
impl<T> WatchableVfsDb for T where
    T: std::ops::Deref<Target = Db> + std::ops::DerefMut + SnapshotClone + Send + 'static
{
}

const DEBOUNCE_TIMEOUT_RAW: u64 = 50;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(DEBOUNCE_TIMEOUT_RAW);

#[cfg(test)]
pub(crate) const DEBOUNCE_TEST_SLEEP_TIME: Duration =
    Duration::from_millis(DEBOUNCE_TIMEOUT_RAW * 4);

pub struct WatchedVfs<DB: WatchableVfsDb> {
    db: Arc<Mutex<DB>>,
    debouncer: Debouncer<RecommendedWatcher>,
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

impl<DB: WatchableVfsDb> WatchedVfs<DB> {
    pub(crate) fn new(mut db: DB) -> Self {
        let db = Arc::new(Mutex::new(db));
        let mut debouncer = new_debouncer(DEBOUNCE_TIMEOUT, {
            let db = Arc::downgrade(&db);
            move |res| match res {
                Ok(events) => process_debounced_events(db.clone(), events),
                Err(e) => todo!("e: {:?}", e),
            }
        })
        .unwrap();
        debouncer
            .watcher()
            .watch(Path::new("."), RecursiveMode::Recursive);
        Self { db, debouncer }
    }
}

fn process_debounced_events<DB: WatchableVfsDb>(db: Weak<Mutex<DB>>, events: Vec<DebouncedEvent>) {
    let Some(db) = db.upgrade() else {
        return;
    };
    for event in events {
        db.lock()
            .unwrap()
            .refresh_file_from_disk(&event.path, /* ad hoc */ Durability::LOW)
            .unwrap()
    }
}

// #[cfg(target_os = "linux")]
#[test]
fn watcher_works() {
    use husky_print_utils::p;
    let db = DB::default();
    let tempdir = tempfile::tempdir_in(".").unwrap();
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
    std::fs::write(&path, "Goodbye, world!").expect("can't write");
    std::thread::sleep(Duration::from_secs(2));
    assert!(db.query(|db| db
        .file_from_virtual_path(abs_path, Durability::LOW)
        .unwrap()
        .content(&db)
        == &FileContent::OnDisk("Goodbye, world!".to_owned())));
}
