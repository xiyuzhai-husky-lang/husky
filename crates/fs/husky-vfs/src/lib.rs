// use timed_salsa::jar::Jar;

mod error;
mod file;
mod path;
mod watcher;

pub use error::*;
pub use watcher::WatchableVfsDb;

use file::*;
use std::path::PathBuf;

#[timed_salsa::interned]
pub struct PathBufItd {
    #[return_ref]
    raw: PathBuf,
}

#[timed_salsa::jar(db = VfsDb)]
pub struct Jar(PathBufItd, HuskyFile);

pub trait VfsDb: timed_salsa::DbWithJar<Jar> + Vfs {}

impl<T> VfsDb for T where T: timed_salsa::DbWithJar<Jar> + Vfs {}

pub trait Vfs {
    fn read_file(&self, path: PathBufItd, class: HuskyFileClass) -> VfsResult<HuskyFile>;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use dashmap::DashMap;
    use husky_print_utils::p;
    use replace_with::replace_with;
    use std::{collections::HashSet, sync::Mutex};

    #[timed_salsa::db(Jar)]
    #[derive(Default)]
    struct VfsTestsDatabase {
        storage: timed_salsa::Storage<Self>,
        files: DashMap<PathBufItd, FileState>,
    }

    pub enum FileState {
        Read(HuskyFile),
        Intact(HuskyFileContent),
    }

    impl VfsTestsDatabase {
        fn write_file(
            &mut self,
            path: PathBufItd,
            class: HuskyFileClass,
            file_content: HuskyFileContent,
        ) -> VfsResult<()> {
            if let Some(mut file_state) = self.files.get_mut(&path) {
                replace_with::<FileState, _, _>(
                    &mut file_state,
                    || todo!(),
                    |file_state| match file_state {
                        FileState::Read(_) => todo!(),
                        FileState::Intact(content) => {
                            todo!()
                            // FileState::Read(HuskyFile::new(self, path, class, content))
                        }
                    },
                );
                Ok(())
            } else {
                self.files.insert(path, FileState::Intact(file_content));
                Ok(())
            }
        }
    }

    impl timed_salsa::Database for VfsTestsDatabase {}

    impl Vfs for VfsTestsDatabase {
        fn read_file(&self, path: PathBufItd, class: HuskyFileClass) -> VfsResult<HuskyFile> {
            if let Some(mut file_state) = self.files.get_mut(&path) {
                replace_with::<FileState, _, _>(
                    &mut file_state,
                    || todo!(),
                    |file_state| match file_state {
                        FileState::Read(_) => file_state,
                        FileState::Intact(content) => {
                            FileState::Read(HuskyFile::new(self, path, class, content))
                        }
                    },
                );
                match file_state.value() {
                    FileState::Read(file) => Ok(*file),
                    FileState::Intact(content) => unreachable!(),
                }
            } else {
                Err(VfsError::FileNotFound(path.raw(self).clone()))
            }
        }
    }

    #[test]
    fn vfs_db_works() {
        let mut db = VfsTestsDatabase::default();
        let path0 = PathBufItd::new(&db, "something".into());
        assert!(db.read_file(path0, HuskyFileClass::Library).is_err());
        db.write_file(
            path0,
            HuskyFileClass::Library,
            HuskyFileContent::new_source_program("bob is cool".to_string()),
        );
        p!(db.read_file(path0, HuskyFileClass::Library));
        assert!(db.read_file(path0, HuskyFileClass::Library).is_ok());
    }
}
