use crate::*;

use husky_file::LiveFiles;

use std::{fs, path::Path};

pub trait ComptimeOps: FileSalsaQuery + ResolveLinkage + Sized {
    fn comptime_config(&self) -> &ComptimeConfig;

    fn set_target_entrance_from_package_dir(&mut self) {
        // assert!(self.opt_target_entrance().is_none());
        let package_main_file =
            self.intern_file(self.comptime_config().package_dir.join("main.hsy"));
        self.set_opt_target_entrance(Some(package_main_file))
    }

    fn load_package(&mut self) {
        self.set_target_entrance_from_package_dir();
        // ad hoc
        self.load_dir(&self.comptime_config().package_dir.clone());
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

    fn load_linkages(&self) {
        self.linkage_table()
            .load(self, &self.comptime_config().package_dir)
    }
}

impl ComptimeOps for HuskyComptime {
    fn comptime_config(&self) -> &ComptimeConfig {
        &self.config
    }
}
