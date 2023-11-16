use crate::*;
use husky_task::helpers::TaskDevRuntimeSpecificConfig;

#[derive(Default)]
pub struct DevRuntimeCommonConfig {}

pub struct DevRuntimeConfig<Task: IsTask> {
    common: DevRuntimeCommonConfig,
    task_specifc: TaskDevRuntimeSpecificConfig<Task>,
}

impl<Task: IsTask> Default for DevRuntimeConfig<Task> {
    fn default() -> Self {
        Self {
            common: Default::default(),
            task_specifc: Default::default(),
        }
    }
}
