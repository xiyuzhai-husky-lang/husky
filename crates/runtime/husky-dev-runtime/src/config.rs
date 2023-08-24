use crate::*;
use husky_task::DevRuntimeTaskSpecificConfig;

#[derive(Default)]
pub struct DevRuntimeCommonConfig {}

pub struct DevRuntimeConfig<Task: IsTask> {
    common: DevRuntimeCommonConfig,
    task_specifc: DevRuntimeTaskSpecificConfig<Task>,
}

impl<Task: IsTask> Default for DevRuntimeConfig<Task> {
    fn default() -> Self {
        Self {
            common: Default::default(),
            task_specifc: Default::default(),
        }
    }
}
