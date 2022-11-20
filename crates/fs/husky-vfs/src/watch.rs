use crate::*;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use dashmap::DashMap;
use eyre::{eyre, Context, Report, Result};
use notify_debouncer_mini::{
    new_debouncer, new_debouncer_opt, notify::RecommendedWatcher, DebounceEventHandler,
    DebounceEventResult, Debouncer,
};
use place::SingleAssignPlace;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub trait HasWatcherPlace {
    fn watcher_place_mut(&mut self) -> &mut SingleAssignPlace<VfsWatcher>;
    fn watcher_place(&self) -> &SingleAssignPlace<VfsWatcher>;
}

pub trait WatchableVfsDb: VfsDb + HasWatcherPlace + Send + 'static {
    fn watcher(&self) -> Option<&VfsWatcher>;
}

impl<T> WatchableVfsDb for T
where
    T: VfsDb + HasWatcherPlace + Send + 'static,
{
    fn watcher(&self) -> Option<&VfsWatcher> {
        self.watcher_place().value()
    }
}

pub struct VfsWatcher(pub(crate) Mutex<Debouncer<RecommendedWatcher>>);

impl VfsWatcher {
    fn new(debounce_tx: Sender<DebounceEventResult>) -> Self {
        VfsWatcher(Mutex::new(
            new_debouncer(Duration::from_secs(1), None, debounce_tx).unwrap(),
        ))
    }
}

pub struct VfsWatcherThread<DB: WatchableVfsDb> {
    event_tx: Sender<VfsWatcherEvent>,
    phantom: PhantomData<DB>,
}

pub struct VfsWatcherInstance<DB: WatchableVfsDb> {
    db: Arc<Mutex<DB>>,
    debounce_rx: Receiver<DebounceEventResult>,
    event_rx: Receiver<VfsWatcherEvent>,
}

pub enum VfsWatcherEvent {
    Close,
}

impl<V: WatchableVfsDb> Drop for VfsWatcherThread<V> {
    fn drop(&mut self) {
        match self.event_tx.send(VfsWatcherEvent::Close) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }
}

impl<V: WatchableVfsDb> VfsWatcherThread<V> {
    pub fn new(db: Arc<Mutex<V>>) -> Self {
        let (event_tx, event_rx) = unbounded();
        let (debounce_tx, debounce_rx) = unbounded();
        db.lock()
            .unwrap()
            .watcher_place_mut()
            .set(VfsWatcher::new(debounce_tx))
            .unwrap();
        thread::spawn(
            || match VfsWatcherInstance::new(db, debounce_rx, event_rx).run() {
                Ok(_) => (),
                Err(_) => todo!(),
            },
        );
        Self {
            event_tx,
            phantom: PhantomData,
        }
    }
}

impl<DB: WatchableVfsDb> VfsWatcherInstance<DB> {
    fn new(
        db: Arc<Mutex<DB>>,
        debounce_rx: Receiver<DebounceEventResult>,
        event_rx: Receiver<VfsWatcherEvent>,
    ) -> Self {
        Self {
            db,
            debounce_rx,
            event_rx,
        }
    }

    fn run(self) -> Result<()> {
        // HELP: GENGTENG
        todo!()
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
}
