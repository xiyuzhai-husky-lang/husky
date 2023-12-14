use crate::*;
use husky_task::helpers::TaskDevAscensionBasePoint;

pub struct DevRuntimeEvaluator<'a, Task: IsTask> {
    runtime: &'a DevRuntime<Task>,
    ascension_base_point: TaskDevAscensionBasePoint<Task>,
}
