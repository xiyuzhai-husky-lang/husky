use crate::*;
use std::{fs, path::Path};

impl HuskyCompileTime {
    pub fn load_package(&mut self, package_dir: &Path) {
        self.load_dir(package_dir);
    }

    fn load_dir(&mut self, dir: &Path) {
        should!(dir.is_dir());
        for maybe_entry in fs::read_dir(dir).unwrap() {
            let path = maybe_entry.expect("what").path();
            if path.is_dir() {
                if path.with_extension("hsk").exists() {
                    self.load_module(&path)
                }
            } else if path.extension().unwrap() == "hsk" {
                let text = fs::read_to_string(&path).expect("what");
                self.set_live_file_text(path, text)
            }
        }
    }

    fn load_module(&mut self, module_dir: &Path) {
        self.load_dir(&module_dir);
    }
}
