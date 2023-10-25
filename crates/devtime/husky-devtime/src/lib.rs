#![feature(try_trait_v2)]
mod state;

use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_task::{helpers::DevLinkTime, IsTask};
use std::path::Path;

use self::state::*;

pub struct Devtime<Task: IsTask> {
    runtime: DevRuntime<Task>,
    state: DevtimeState,
}

impl<Task: IsTask> Default for Devtime<Task>
where
    Task: Default,
    DevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            runtime: Default::default(),
            state: Default::default(),
        }
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}

impl<Task: IsTask> Devtime<Task> {
    pub fn new(
        task: Task,
        target_crate: &Path,
        runtime_config: Option<DevRuntimeConfig<Task>>,
    ) -> Self {
        Self {
            runtime: DevRuntime::new(task, target_crate, runtime_config),
            state: Default::default(),
        }
    }
}
