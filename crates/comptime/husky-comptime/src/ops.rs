use crate::*;

use husky_path::VfsQueryGroupBase;

use std::{fs, path::Path};

pub trait ComptimeOps: FileSalsaQuery + ResolveLinkage + Sized {
    fn comptime_config(&self) -> &ComptimeConfig;

    fn set_target_entrance_from_package_dir(&mut self) {
        // assert!(self.opt_target_entrance().is_none());
        let package_main_file =
            self.intern_path(self.comptime_config().package_dir.join("main.hsy"));
        self.set_opt_target_entrance(Some(package_main_file))
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
