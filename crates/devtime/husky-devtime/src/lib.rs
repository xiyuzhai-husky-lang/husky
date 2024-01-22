#![feature(try_trait_v2)]
mod state;

pub use husky_trace_protocol::server::IsTracetime;
use husky_visual_protocol::synchrotron::VisualSynchrotron;

use self::state::*;
use husky_dev_comptime::DevComptimeTarget;
use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_task::{
    dev_ascension::IsDevAscension,
    helpers::{TaskDevLinkTime, TaskTraceProtocol},
    IsTask,
};
use husky_trace::{db::TraceDb, trace::Trace};
use husky_trace_protocol::{
    id::AccompanyingTraceIds,
    protocol::{IsTraceProtocol, TraceBundle},
    server::ValVisualCache,
    stalk::TraceStalk,
    synchrotron::AccompanyingTraceIdsExceptFollowed,
};
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, ValuePresenterCache,
};
use husky_vfs::error::VfsResult;
use std::path::Path;

pub struct Devtime<Task: IsTask> {
    runtime: DevRuntime<Task>,
}

impl<Task: IsTask> Devtime<Task> {
    pub fn new(
        task: Task,
        target_crate: &Path,
        runtime_config: Option<DevRuntimeConfig<Task>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            runtime: DevRuntime::new(task, target_crate, runtime_config)?,
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
        }
    }
}

impl<Task: IsTask> IsTracetime for Devtime<Task> {
    type Trace = Trace;

    type TraceProtocol = TaskTraceProtocol<Task>;

    type SerdeImpl = serde_impl::json::SerdeJson;

    fn get_trace_bundles(&self) -> &[TraceBundle<Self::Trace>] {
        match self.target() {
            DevComptimeTarget::None => &[],
            DevComptimeTarget::SingleCrate(crate_path) => self.db().trace_bundles(crate_path),
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
        trace: Self::Trace,
        pedestal: <Self::TraceProtocol as IsTraceProtocol>::Pedestal,
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

    fn get_figure(
        &self,
        followed_trace: Option<Self::Trace>,
        accompanying_trace_ids_expect_followed: &AccompanyingTraceIdsExceptFollowed,
        pedestal: <Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<<Self::TraceProtocol as IsTraceProtocol>::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        let db = self.runtime.db();
        let followed_trace_id_val_repr_pair = match followed_trace {
            Some(followed_trace) => followed_trace
                .val_repr(db)
                .map(|val_repr| (followed_trace.into(), val_repr.into())),
            None => None,
        };
        let accompanying_trace_id_val_repr_pairs = accompanying_trace_ids_expect_followed
            .iter()
            .filter_map(|&accompanying_trace_id| {
                let trace: Trace = accompanying_trace_id.into();
                Some((trace.into(), trace.val_repr(db)?.into()))
            })
            .collect();
        <Task::DevAscension as IsDevAscension>::calc_figure(
            followed_trace_id_val_repr_pair,
            accompanying_trace_id_val_repr_pairs,
            pedestal,
            &self.runtime,
            visual_synchrotron,
            val_visual_cache,
        )
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
