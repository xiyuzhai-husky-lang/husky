#![feature(try_trait_v2)]
mod state;

pub use husky_trace_protocol::server::IsTracetime;

use self::state::*;
use husky_dev_comptime::DevComptimeTarget;
use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_task::{
    helpers::{TaskDevLinkTime, TaskTraceProtocol},
    IsTask,
};
use husky_trace::{db::TraceDb, trace::Trace};
use husky_trace_protocol::stalk::TraceStalk;
use husky_value_protocol::presentation::ValuePresentationSynchrotron;
use husky_value_protocol::presentation::ValuePresenterCache;
use husky_vfs::error::VfsResult;
use std::path::Path;

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
    type Trace = Trace;

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

    fn get_trace_stalk(
        &self,
        pedestal: <Self::TraceProtocol as husky_trace_protocol::protocol::IsTraceProtocol>::Pedestal,
        trace: Self::Trace,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> husky_trace_protocol::stalk::TraceStalk {
        let db = self.runtime.db();
        if let Some(val_repr) = trace.val_repr(db) {
            TraceStalk::Val(
                self.runtime
                    .eval_val_repr_at_pedestal(val_repr, pedestal)
                    .present(value_presenter_cache, value_presentation_synchrotron),
            )
        } else {
            // ad hoc
            TraceStalk::None
        }
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
