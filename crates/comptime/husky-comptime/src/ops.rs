use crate::*;
use __husky::init::__StaticLinkageKey;
use husky_file::LiveFiles;
use husky_vm::__Linkage;
use std::{fs, path::Path};

pub trait ComptimeOps: FileSalsaQuery + ResolveLinkage + Sized {
    fn set_target_entrance_from_package_dir(&mut self, package_dir: &Path) {
        // assert!(self.opt_target_entrance().is_none());
        self.set_opt_target_entrance(Some(self.intern_file(package_dir.join("main.hsy"))))
    }

    fn load_package(&mut self, package_dir: &Path) {
        self.set_target_entrance_from_package_dir(package_dir);
        self.load_dir(package_dir);
    }

    fn load_dir(&mut self, dir: &Path) {
        should_satisfy!(dir, |dir: &Path| dir.is_dir());
        for maybe_entry in fs::read_dir(dir).unwrap() {
            let path = maybe_entry.expect("what").path();
            if path.is_dir() {
                if path.with_extension("hsy").exists() {
                    self.load_module(&path)
                }
            } else if path.extension().unwrap() == "hsy" {
                let text = fs::read_to_string(&path).expect("what");
                self.set_live_file_text(path, text)
            }
        }
    }

    fn load_module(&mut self, module_dir: &Path) {
        self.load_dir(&module_dir);
    }

    fn load_linkages(&self, linkages: &[(__StaticLinkageKey, __Linkage)]) {
        self.linkage_table().load(self, linkages)
    }
}

impl ComptimeOps for Comptime {}
