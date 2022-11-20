use crate::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use dashmap::DashMap;
use eyre::{eyre, Context, Report, Result};
use notify_debouncer_mini::{notify::RecommendedWatcher, Debouncer};
use place::SingleAssignPlace;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread,
};

pub trait HasWatcherPlace {
    fn watcher_place(&mut self) -> &mut SingleAssignPlace<VfsWatcher>;
}

pub trait WatchableVfsDb: VfsDb + HasWatcherPlace + Send + 'static {
    fn watcher(&self) -> Option<&VfsWatcher>;
}

impl<T> WatchableVfsDb for T
where
    T: VfsDb + HasWatcherPlace + Send + 'static,
{
    fn watcher(&self) -> Option<&VfsWatcher> {
        todo!()
    }
}

pub struct VfsWatcher;

impl VfsWatcher {
    fn new(tx: Sender<VfsWatcherEvent>) -> Self {
        VfsWatcher
    }
}

pub struct VfsWatcherThread<DB: WatchableVfsDb> {
    tx: Sender<VfsWatcherEvent>,
    phantom: PhantomData<DB>,
}

pub struct VfsWatcherInstance<DB: WatchableVfsDb> {
    db: Arc<Mutex<DB>>,
    rx: Receiver<VfsWatcherEvent>,
}

pub enum VfsWatcherEvent {
    Close,
}

impl<V: WatchableVfsDb> Drop for VfsWatcherThread<V> {
    fn drop(&mut self) {
        match self.tx.send(VfsWatcherEvent::Close) {
            Ok(_) => (),
            Err(_) => todo!(),
        }
    }
}

impl<V: WatchableVfsDb> VfsWatcherThread<V> {
    pub fn new(db: Arc<Mutex<V>>) -> Self {
        let (tx, rx) = unbounded();
        db.lock()
            .unwrap()
            .watcher_place()
            .set(VfsWatcher::new(tx.clone()))
            .unwrap();
        thread::spawn(|| match VfsWatcherInstance::new(db, rx).run() {
            Ok(_) => (),
            Err(_) => todo!(),
        });
        Self {
            tx,
            phantom: PhantomData,
        }
    }
}

impl<DB: WatchableVfsDb> VfsWatcherInstance<DB> {
    fn new(db: Arc<Mutex<DB>>, rx: Receiver<VfsWatcherEvent>) -> Self {
        Self { db, rx }
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
