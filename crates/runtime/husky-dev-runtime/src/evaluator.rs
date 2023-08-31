use crate::*;
use husky_task::helpers::DevAscensionBase;

pub struct DevRuntimeEvaluator<'a, Task: IsTask> {
    runtime: &'a DevRuntime<Task>,
    ascension_base_point: DevAscensionBase<Task>,
}
