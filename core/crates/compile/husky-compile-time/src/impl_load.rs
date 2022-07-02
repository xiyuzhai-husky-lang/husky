use static_defn::__StaticLinkageKey;
use vm::__Linkage;

use crate::*;
use std::{fs, path::Path};

impl HuskyCompileTime {
    pub fn set_main_package(&mut self, dir: &Path) {
        assert!(self.opt_main.is_none());
        self.opt_main = Some(self.intern_file(dir.join("main.hsk")))
    }

    pub fn load_package(&mut self, package_dir: &Path) {
        self.load_dir(package_dir);
    }

    fn load_dir(&mut self, dir: &Path) {
        p!(dir);
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

    pub fn load_linkages(&self, linkages: &'static [(__StaticLinkageKey, __Linkage)]) {
        todo!()
    }
}
