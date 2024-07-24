#![feature(try_trait_v2)]
mod state;

pub use husky_trace_protocol::server::IsTracetime;
use husky_visual_protocol::synchrotron::VisualSynchrotron;

use husky_dev_comptime::DevComptimeTarget;
use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_devsoul::devsoul::IsDevsoul;
use husky_trace::{jar::TraceDb, trace::Trace};
use husky_trace_protocol::{
    protocol::{IsTraceProtocol, TraceBundle},
    server::ValVisualCache,
    stalk::TraceStalk,
    synchrotron::accompany::AccompanyingTraceIdsExceptFollowed,
};
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, ValuePresenterCache,
};
use husky_vfs::error::VfsResult;
use std::{path::Path, pin::Pin};

pub struct Devtime<Devsoul: IsDevsoul> {
    runtime: Pin<Box<DevRuntime<Devsoul>>>,
}

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn new(
        target_crate: &Path,
        runtime_config: Option<DevRuntimeConfig<Devsoul>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            runtime: DevRuntime::new(target_crate, runtime_config)?,
        })
    }

    pub fn db(&self) -> &::salsa::Db {
        self.runtime.db()
    }

    pub fn target(&self) -> DevComptimeTarget {
        self.runtime.comptime_target()
    }
}

// impl<Devsoul: IsDevsoul> Default for Devtime<Devsoul>
// where
//     Devsoul: Default,
//     Devsoul::Linktime: Default,
// {
//     fn default() -> Self {
//         Self {
//             runtime: Default::default(),
//         }
//     }
// }

impl<Devsoul: IsDevsoul> IsTracetime for Devtime<Devsoul> {
    type Trace = Trace;

    type TraceProtocol = Devsoul::TraceProtocol;

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
        pedestal: &<Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> husky_trace_protocol::stalk::TraceStalk {
        use husky_devsoul_interface::pedestal::IsPedestal;
        let db = self.runtime.db();
        if !pedestal.is_closed() {
            return TraceStalk::None;
        }
        if let Some(ki_repr) = trace.ki_repr(db) {
            TraceStalk::Ki(
                self.runtime
                    .eval_ki_repr(ki_repr)
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
        let followed = match followed_trace {
            Some(followed_trace) => followed_trace.ki_repr(db).map(|ki_repr| {
                (
                    followed_trace.into(),
                    ki_repr.into(),
                    ki_repr.ki_domain_repr(db).into(),
                )
            }),
            None => None,
        };
        let accompanyings_except_followed = &accompanying_trace_ids_expect_followed
            .iter()
            .filter_map(|&accompanying_trace_id| {
                let trace: Trace = accompanying_trace_id.into();
                Some((trace.into(), trace.ki_repr(db)?.into()))
            })
            .collect::<Vec<_>>();
        Devsoul::calc_figure(
            followed,
            accompanyings_except_followed,
            pedestal,
            &*self.runtime,
            visual_synchrotron,
            val_visual_cache,
        )
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
