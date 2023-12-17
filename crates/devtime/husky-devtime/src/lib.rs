#![feature(try_trait_v2)]
mod state;

use husky_dev_comptime::DevComptimeTarget;
pub use husky_trace_protocol::server::IsTracetime;

use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_task::{
    helpers::{TaskDevLinkTime, TaskTraceProtocol},
    IsTask,
};
use husky_trace::{db::TraceDb, trace::TraceId};
use husky_vfs::error::VfsResult;
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
    ) -> VfsResult<Self> {
        Ok(Self {
            runtime: DevRuntime::new(task, target_crate, runtime_config)?,
            state: Default::default(),
        })
    }

    pub fn db(&self) -> &::salsa::Db {
        self.runtime.db()
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.runtime.comptime_target()
    }
}

impl<Task: IsTask> Default for Devtime<Task>
where
    Task: Default,
    TaskDevLinkTime<Task>: Default,
{
    fn default() -> Self {
        Self {
            runtime: Default::default(),
            state: Default::default(),
        }
    }
}

impl<Task: IsTask> IsTracetime for Devtime<Task> {
    type Trace = TraceId;

    type TraceProtocol = TaskTraceProtocol<Task>;

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
