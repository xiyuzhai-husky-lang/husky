#![feature(try_trait_v2)]
mod state;

use husky_dev_comptime::DevComptimeTarget;
pub use husky_trace_protocol::server::IsTracetime;

use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_task::{
    helpers::DevLinkTime,
    visual::{VisualComponent, VisualProtocol},
    DevComptimeDb, IsTask,
};
use husky_trace::{db::TraceDb, Trace};
use std::path::Path;

use self::state::*;

pub struct Devtime<Task: IsTask>
where
    DevComptimeDb<Task>: TraceDb,
{
    runtime: DevRuntime<Task>,
    state: DevtimeState,
}

impl<Task: IsTask> Devtime<Task>
where
    DevComptimeDb<Task>: TraceDb,
{
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

    pub fn db(&self) -> &DevComptimeDb<Task> {
        self.runtime.db()
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.runtime.target()
    }
}

impl<Task: IsTask> Default for Devtime<Task>
where
    Task: Default,
    DevLinkTime<Task>: Default,
    DevComptimeDb<Task>: TraceDb,
{
    fn default() -> Self {
        Self {
            runtime: Default::default(),
            state: Default::default(),
        }
    }
}

impl<Task: IsTask> IsTracetime for Devtime<Task>
where
    DevComptimeDb<Task>: TraceDb,
{
    type Trace = Trace;

    type VisualComponent = VisualComponent<Task>;

    type SerdeImpl = serde_impl::json::SerdeJson;

    fn get_root_traces(&self) -> &[Self::Trace] {
        match self.target() {
            DevComptimeTarget::None => &[],
            DevComptimeTarget::SingleCrate(crate_path) => self.db().root_traces(crate_path),
        }
    }

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace] {
        trace.subtraces(self.db())
    }

    fn get_trace_view_data(&self, trace: Self::Trace) -> husky_trace_protocol::view::TraceViewData {
        trace.view_data(self.db())
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
