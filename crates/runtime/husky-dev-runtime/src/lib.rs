#![feature(try_trait_v2)]

mod config;
mod evaluator;
mod hot_reload;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_task::{dev_ascension::with_eval_context, helpers::TaskDevAscension};
use husky_task::{
    helpers::{DevRuntimeStorage, TaskDevAscensionBasePoint, TaskDevLinkTime},
    IsTask,
};
use husky_task_prelude::{IsDevRuntime, IsDevRuntimeDyn};
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

    pub fn db(&self) -> &::salsa::Db {
        self.comptime.db()
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }

    pub fn with_eval_context<R>(
        &self,
        base_point: TaskDevAscensionBasePoint<Task>,
        f: impl FnOnce() -> R,
    ) -> R {
        with_eval_context::<TaskDevAscension<Task>, _, _>(self, base_point, f)
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

impl<Task: IsTask> IsDevRuntime<TaskDevAscensionBasePoint<Task>> for DevRuntime<Task> {
    type StaticSelf = Self;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf {
        &*(unsafe { self as *const _ })
    }
}
