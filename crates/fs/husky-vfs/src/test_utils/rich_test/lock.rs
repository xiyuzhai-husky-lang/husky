use std::{fs::File, io::Write, path::PathBuf};

use super::VfsTestConfig;

pub struct RichTestLock {
    lock_path: PathBuf,
    lock_file: Option<File>,
}

impl RichTestLock {
    pub fn new(config: &VfsTestConfig) -> Self {
        let lock_path = config
            .cargo_manifest_dir()
            .join(config.test_name())
            .with_extension("rich-test-lock");
        let lock_file = if std::env::var("UPDATE_EXPECT=1").is_ok() {
            Some(match std::fs::File::create_new(&lock_path) {
                Ok(lock_file) => lock_file,
                Err(e) => panic!(
                    "failed to lock path `{:?}` for task `{}` due to {e}",
                    lock_path,
                    config.test_name()
                ),
            })
        } else {
            None
        };
        Self {
            lock_path,
            lock_file,
        }
    }
}

impl std::ops::Drop for RichTestLock {
    fn drop(&mut self) {
        if let Some(ref mut lock_file) = self.lock_file {
            lock_file.write(b"").unwrap();
        }
    }
}
