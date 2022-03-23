use std::path::{Path, PathBuf};

#[salsa::query_group(VfsDatabaseStorage)]
trait VfsDatabase: salsa::Database + FileWatcher {
    fn read(&self, path: PathBuf) -> String;
}

trait FileWatcher {
    fn watch(&self, path: &Path);
    fn did_change_file(&mut self, path: &PathBuf);
}

fn read(db: &dyn VfsDatabase, path: PathBuf) -> String {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    db.watch(&path);
    std::fs::read_to_string(&path).unwrap_or_default()
}

#[salsa::database(VfsDatabaseStorage)]
struct MyDatabase {
    storage: salsa::Storage<MyDatabase>,
}

impl salsa::Database for MyDatabase {}

impl FileWatcher for MyDatabase {
    fn watch(&self, _path: &Path) {
        todo!()
    }
    fn did_change_file(&mut self, path: &PathBuf) {
        ReadQuery.in_db_mut(self).invalidate(path);
    }
}

#[test]
fn change_file() {}
