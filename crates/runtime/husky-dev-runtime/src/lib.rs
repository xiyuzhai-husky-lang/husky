#![feature(try_trait_v2)]

mod config;
mod db;
mod evaluator;
mod hot_reload;
mod impl_necessary;
mod impl_train;

pub use self::config::*;

use self::db::*;
use husky_check_utils::*;
use husky_compiler::CompilerInstance;
use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_eval::*;
use husky_print_utils::*;
use husky_task::{
    helpers::{DevLinkTime, DevRuntimeStorage},
    DevComptimeDb, IsTask,
};
use husky_vfs::{CratePath, DiffPathBuf};
use indexmap::IndexMap;
use relative_path::RelativePathBuf;
use std::{path::Path, sync::Arc};
use sync_utils::ASafeRwLock;

pub struct DevRuntime<Task: IsTask> {
    task: Task,
    comptime: DevComptime<Task>,
    storage: DevRuntimeStorage<Task>,
    config: DevRuntimeConfig<Task>,
}

impl<Task: IsTask> DevRuntime<Task> {
    pub fn new(task: Task, target_crate: &Path, config: Option<DevRuntimeConfig<Task>>) -> Self {
        Self {
            task,
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate),
        }
    }

    pub fn db(&self) -> &DevComptimeDb<Task> {
        self.comptime.db()
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }
}

impl<Task: IsTask> Default for DevRuntime<Task>
where
    Task: Default,
    DevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            task: Default::default(),
            comptime: Default::default(),
            storage: Default::default(),
            config: Default::default(),
        }
    }
}
