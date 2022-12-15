use crate::{
    watch::{WatchedVfs, DEBOUNCE_TEST_SLEEP_TIME},
    *,
};
use crossbeam_channel::{unbounded, Sender};
use dashmap::DashMap;
use husky_absolute_path::{AbsolutePath, AbsolutePathJar};
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_package_path::{PackagePathData, PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_source_path::{
    HasSourcePathConfig, SourcePath, SourcePathConfig, SourcePathConfigMimic, SourcePathData,
    SourcePathJar,
};
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use place::SingleAssignPlace;
use replace_with::replace_with;
use salsa::Durability;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

#[salsa::db(
    VfsJar,
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    AbsolutePathJar,
    SourcePathJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
    watcher_place: SingleAssignPlace<VfsWatcher>,
    vfs_config: SourcePathConfigMimic,
}

impl salsa::Database for MimicDB {}

impl HasSourcePathConfig for MimicDB {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.vfs_config
    }
}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(MimicDB {
            storage: self.storage.snapshot(),
            watcher_place: self.watcher_place.clone(),
            vfs_config: self.vfs_config.clone(),
        })
    }
}

#[test]
fn watcher_works() {
    let db = MimicDB::default();
    let db = WatchedVfs::new(db);
    let tempdir = tempfile::tempdir().unwrap();
    let some_pkg_dir = tempdir.path().join("somepath");
    std::fs::create_dir(&some_pkg_dir).unwrap();
    let path = some_pkg_dir.join("Corgi.toml");
    let abs_path: AbsolutePath = db
        .apply(|db| AbsolutePath::new_from_owned(db, path.clone()))
        .unwrap();
    unsafe {
        std::fs::write(&path, "Hello, world!");
        assert!(db.apply(|db| db.file_content(abs_path, Durability::LOW)
            == &FileContent::OnDisk("Hello, world!".to_owned())),);
        std::fs::write(&path, "Hello, world!2");
        let a = DEBOUNCE_TEST_SLEEP_TIME;
        std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
        assert!(db.apply(|db| db.file_content(abs_path, Durability::LOW)
            == &FileContent::OnDisk("Hello, world!2".to_owned())))
    }
}

#[test]
fn resolve_source_path_works() {
    fn t(db: &MimicDB, src_path: SourcePath) {
        let abs_path = db.absolute_path_from_source_path(src_path).unwrap();
        assert_eq!(
            src_path,
            db.source_path_from_absolute_path(abs_path)
                .unwrap()
                .unwrap()
        );
    }

    let db = MimicDB::default();
    let entity_path_menu = db.entity_path_menu();
    t(
        &db,
        db.it_source_path(SourcePathData::Module(entity_path_menu.core())),
    )
}
