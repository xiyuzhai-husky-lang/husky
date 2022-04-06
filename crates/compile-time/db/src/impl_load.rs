use crate::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn file_should_exists(dir: PathBuf, filename: &str) {
    let filepath = {
        let mut filepath = dir;
        filepath.push(filename);
        filepath
    };
    assert!(filepath.exists())
}

impl HuskyLangCompileTime {
    pub fn load_pack(&mut self, pack_dir: PathBuf) {
        self.load_dir(&pack_dir);
        file_should_exists(pack_dir, "main.hsk")
    }

    fn load_dir(&mut self, dir: &Path) {
        should!(dir.is_dir());
        for maybe_entry in fs::read_dir(dir).unwrap() {
            let path = maybe_entry.expect("what").path();
            if path.is_dir() {
                if path.join("mod.hsk").exists() {
                    self.load_module(path)
                }
            } else if path.extension().unwrap() == "hsk" {
                let text = fs::read_to_string(&path).expect("what");
                if text.len() == 0 {
                    panic!("{} is empty", &path.as_os_str().to_str().unwrap());
                }
                self.set_live_file_text(path, text)
            }
        }
    }

    fn load_module(&mut self, module_dir: PathBuf) {
        self.load_dir(&module_dir);
        file_should_exists(module_dir, "mod.hsk")
    }
}
