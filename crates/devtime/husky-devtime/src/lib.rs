#![feature(try_trait_v2)]
mod state;

use husky_dev_comptime::db::DevComptimeDb;
use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_task::{helpers::DevLinkTime, visual::VisualProtocol, IsTask};
use husky_trace::Trace;
use husky_trace_protocol::server::IsTracetime;
use std::path::Path;

use self::state::*;

pub struct Devtime<Task: IsTask> {
    runtime: DevRuntime<Task>,
    state: DevtimeState,
}

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

    pub fn db(&self) -> &DevComptimeDb {
        self.runtime.db()
    }
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

impl<Task: IsTask> IsTracetime for Devtime<Task> {
    type Trace = Trace;

    type VisualProtocol = VisualProtocol<Task>;

    type SerdeImpl = serde_impl::json::SerdeJson;

    fn get_root_traces(&self) -> &[Self::Trace] {
        todo!()
    }

    fn get_subtraces(&self, trace: Self::Trace) -> &[Self::Trace] {
        todo!()
    }

    fn get_trace_view_data(
        &self,
        trace: Self::Trace,
    ) -> &husky_trace_protocol::view::TraceViewData {
        todo!()
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
