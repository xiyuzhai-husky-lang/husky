#![feature(try_trait_v2)]

mod config;
mod evaluator;
mod hot_reload;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};

use husky_task::{
    helpers::TaskDevComptimeDb,
    helpers::{DevRuntimeStorage, TaskDevLinkTime},
    IsTask,
};
use husky_vfs::error::VfsResult;

use std::path::Path;

pub struct DevRuntime<Task: IsTask> {
    task: Task,
    comptime: DevComptime<Task>,
    storage: DevRuntimeStorage<Task>,
    config: DevRuntimeConfig<Task>,
}

impl<Task: IsTask> DevRuntime<Task> {
    pub fn new(
        task: Task,
        target_crate: &Path,
        config: Option<DevRuntimeConfig<Task>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            task,
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate)?,
        })
    }

    pub fn db(&self) -> &TaskDevComptimeDb<Task> {
        self.comptime.db()
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }
}

impl<Task: IsTask> Default for DevRuntime<Task>
where
    Task: Default,
    TaskDevLinkTime<Task>: Default,
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
