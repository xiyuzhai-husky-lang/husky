use crate::*;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use eyre::Result;
use notify_debouncer_mini::{
    new_debouncer, notify::RecommendedWatcher, DebounceEventResult, DebouncedEvent, Debouncer,
};
use salsa::{ParallelDatabase, Snapshot};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub trait WatchableVfsDb: VfsDb + Send {
    fn watcher(&self) -> Option<&VfsWatcher>;
}

impl<T> WatchableVfsDb for T
where
    T: VfsDb + Send,
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
            new_debouncer(DEBOUNCE_TIMEOUT, None, debounce_tx).unwrap(),
        )))
    }
}

const DEBOUNCE_TIMEOUT_RAW: u64 = 50;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(DEBOUNCE_TIMEOUT_RAW);

#[cfg(test)]
pub(crate) const DEBOUNCE_TEST_SLEEP_TIME: Duration =
    Duration::from_millis(DEBOUNCE_TIMEOUT_RAW * 4);

pub struct WatchedVfs<DB: WatchableVfsDb>
where
    DB: ParallelDatabase,
{
    event_tx: Sender<VfsWatcherEvent>,
    snapshot_rx: Receiver<Snapshot<DB>>,
    phantom: PhantomData<DB>,
}

impl<DB> Default for WatchedVfs<DB>
where
    DB: Default + WatchableVfsDb + ParallelDatabase + 'static,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<DB: WatchableVfsDb> WatchedVfs<DB>
where
    DB: ParallelDatabase,
{
    pub fn query<S>(&self, f: impl FnOnce(salsa::Snapshot<DB>) -> S) -> S {
        self.event_tx.send(VfsWatcherEvent::Snapshot);
        match self.snapshot_rx.recv() {
            Ok(snapshot) => f(snapshot),
            Err(e) => {
                p!(e);
                todo!()
            }
        }
    }
}

pub struct VfsWatcherInstance<DB: WatchableVfsDb>
where
    DB: ParallelDatabase,
{
    db: DB,
    debounce_rx: Receiver<DebounceEventResult>,
    event_rx: Receiver<VfsWatcherEvent>,
    snapshot_tx: Sender<Snapshot<DB>>,
}

pub enum VfsWatcherEvent {
    Snapshot,
    Close,
}

impl<V: WatchableVfsDb> Drop for WatchedVfs<V>
where
    V: ParallelDatabase,
{
    fn drop(&mut self) {
        match self.event_tx.send(VfsWatcherEvent::Close) {
            Ok(_) => (),
            Err(e) => eprintln!("error {e} in sending VfsWatcherEvent::Close"),
        }
    }
}

impl<DB: WatchableVfsDb> WatchedVfs<DB>
where
    DB: ParallelDatabase + 'static,
{
    pub(crate) fn new(mut db: DB) -> Self {
        let (event_tx, event_rx) = unbounded();
        let (debounce_tx, debounce_rx) = unbounded();
        let (snapshot_tx, snapshot_rx) = unbounded();
        db.vfs_jar_mut().set_watcher(VfsWatcher::new(debounce_tx));
        thread::spawn(|| {
            match VfsWatcherInstance::new(db, debounce_rx, event_rx, snapshot_tx).run() {
                Ok(_) => (),
                Err(_) => todo!(),
            }
        });
        Self {
            event_tx,
            snapshot_rx,
            phantom: PhantomData,
        }
    }
}

impl<DB: WatchableVfsDb> VfsWatcherInstance<DB>
where
    DB: ParallelDatabase + 'static,
{
    fn new(
        db: DB,
        debounce_rx: Receiver<DebounceEventResult>,
        event_rx: Receiver<VfsWatcherEvent>,
        snapshot_tx: Sender<Snapshot<DB>>,
    ) -> Self {
        Self {
            db,
            debounce_rx,
            event_rx,
            snapshot_tx,
        }
    }

    fn run(mut self) -> Result<()> {
        // HELP: GENGTENG
        loop {
            select! {
                recv(self.debounce_rx) -> debounce => match debounce {
                        Ok(Ok(debounced_events)) => self.process_debounced_events(debounced_events),
                        Ok(Err(_)) => todo!(),
                        Err(_) => todo!(),
                    },
                recv(self.event_rx) -> event_result => match event_result {
                    Ok(event) => match event {
                        VfsWatcherEvent::Snapshot => match self.snapshot_tx.send(self.db.snapshot()) {
                            Ok(_) => (),
                            Err(_) => todo!(),
                        },
                        VfsWatcherEvent::Close => return Ok(()),
                    },
                    Err(_) => todo!(),
                },
            }
        }
        // let initial_file_path = std::env::args_os()
        //     .nth(1)
        //     .ok_or_else(|| eyre!("Usage: ./lazy-input <input-file>"))?;

        // // Create the initial input using the input method so that changes to it
        // // will be watched like the other files.
        // let initial = db.input(initial_file_path.into())?;
        // loop {
        //     // Compile the code starting at the provided input, this will read other
        //     // needed files using the on-demand mechanism.
        //     let sum = compile(&db, initial);
        //     let diagnostics = compile::accumulated::<Diagnostic>(&db, initial);
        //     if diagnostics.is_empty() {
        //         println!("Sum is: {}", sum);
        //     } else {
        //         for diagnostic in diagnostics {
        //             println!("{}", diagnostic);
        //         }
        //     }

        //     for log in db.logs.lock().unwrap().drain(..) {
        //         eprintln!("{}", log);
        //     }

        //     // Wait for file change events, the output can't change unless the
        //     // inputs change.
        //     for event in rx.recv()?.unwrap() {
        //         let path = event.path.canonicalize().wrap_err_with(|| {
        //             format!("Failed to canonicalize path {}", event.path.display())
        //         })?;
        //         let file = match db.files.get(&path) {
        //             Some(file) => *file,
        //             None => continue,
        //         };
        //         // `path` has changed, so read it and update the contents to match.
        //         // This creates a new revision and causes the incremental algorithm
        //         // to kick in, just like any other update to a salsa input.
        //         let contents = std::fs::read_to_string(path)
        //             .wrap_err_with(|| format!("Failed to read file {}", event.path.display()))?;
        //         file.set_contents(&mut db).to(contents);
        //     }
        // }
    }

    fn process_debounced_events(&mut self, events: Vec<DebouncedEvent>) {
        for event in events {
            self.db.update_file(&event.path).unwrap()
        }
    }
}
