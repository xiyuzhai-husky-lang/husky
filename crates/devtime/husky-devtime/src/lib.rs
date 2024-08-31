#![feature(result_flattening)]
#![feature(try_trait_v2)]
pub mod eval;
mod state;
#[cfg(test)]
mod tests;

pub use husky_trace_protocol::server::IsTracetime;

use husky_dev_comptime::DevComptimeTarget;
use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_devsoul::{
    devsoul::IsDevsoul,
    helpers::{DevsoulCaryatid, DevsoulChart, DevsoulFigure, DevsoulStaticVarResult},
};
use husky_entity_path::path::ItemPathId;
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr::repr::KiRepr;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface};
use husky_linket_impl::eval_context::IsDevRuntime;
use husky_trace::{jar::TraceDb, trace::Trace};
use husky_trace_protocol::{
    caryatid::IsCaryatid,
    figure::{IsFigure, TraceFigureKey},
    item_path::ItemPathPresentation,
    protocol::{IsTraceProtocol, TraceBundle, TraceVarId},
    server::TraceVisualCache,
    stalk::TraceStalk,
    synchrotron::accompany::AccompanyingTraceIdsExceptFollowed,
    trace_id::TraceId,
    var_id::VarIdPresentation,
    windlass::Windlass,
};
use husky_value_interface::ki_control_flow::KiControlFlow;
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, ValuePresenterCache,
};
use husky_vfs::error::VfsResult;
use husky_visual_protocol::{
    synchrotron::VisualSynchrotron,
    visual::{CompositeVisual, Visual},
};
use salsa::DebugWithDb;
use smallvec::{SmallVec, ToSmallVec};
use std::{path::Path, pin::Pin};
use vec_like::SmallVecSet;

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

impl<Devsoul: IsDevsoul> IsTracetime for Devtime<Devsoul> {
    type Trace = Trace;

    type TraceProtocol = Devsoul::TraceProtocol;

    type SerdeImpl = serde_impl::json::SerdeJson;

    fn trace_bundles(&self) -> &[TraceBundle<Self::Trace>] {
        match self.target() {
            DevComptimeTarget::None => &[],
            DevComptimeTarget::SingleCrate(crate_path) => self.db().trace_bundles(crate_path),
        }
    }

    fn subtraces(&self, trace: Self::Trace) -> &[Self::Trace] {
        trace.subtraces(self.db())
    }

    fn trace_var_deps(&self, trace: Self::Trace) -> SmallVec<[ItemPathIdInterface; 2]> {
        trace.var_deps(self.db()).to_smallvec()
    }

    fn trace_view_data(&self, trace: Self::Trace) -> husky_trace_protocol::view::TraceViewData {
        trace.view_data(self.db())
    }

    fn calc_trace_stalk(
        &self,
        trace: Self::Trace,
        pedestal: &<Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> husky_trace_protocol::stalk::TraceStalk {
        use husky_linket_impl::pedestal::IsPedestal;
        let db = self.runtime.db();
        let var_deps = trace.var_deps(db);
        if !pedestal.is_closed(var_deps) {
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

    fn calc_figure(
        &self,
        figure_key: &TraceFigureKey<Devsoul::TraceProtocol>,
        visual_synchrotron: &mut VisualSynchrotron,
        trace_visual_cache: &mut TraceVisualCache<
            <Self::TraceProtocol as IsTraceProtocol>::Pedestal,
        >,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        let db = self.runtime.db();
        let chart: Option<DevsoulChart<Devsoul, CompositeVisual<TraceId>>> =
            self.runtime.with_var_anchors(
                figure_key.joint_static_var_anchors().iter().copied(),
                |joint_pedestal| {
                    let mut t = |trace_id: TraceId| {
                        let trace: Trace = trace_id.into();
                        let var_deps = trace.var_deps(db);
                        let pedestal = joint_pedestal.pedestal(var_deps);
                        self.trace_visual(trace, pedestal, visual_synchrotron, trace_visual_cache)
                            .map(|visual| (trace_id, visual))
                    };
                    Some(CompositeVisual {
                        followed_reduced: match figure_key.followed_reduced() {
                            Some(followed_reduced) => Some(t(followed_reduced)?),
                            None => None,
                        },
                        accompanyings_except_followed_reduced: figure_key
                            .accompanyings_except_followed_reduced()
                            .iter()
                            .copied()
                            .filter_map(t)
                            .collect(),
                    })
                },
            );
        let trace_plot_map = trace_visual_cache.calc_plots(figure_key.traces().collect());
        IsFigure::from_chart(chart, trace_plot_map, visual_synchrotron)
    }

    fn calc_item_path_presentations(
        &self,
        item_path_id_interface: ItemPathIdInterface,
    ) -> ItemPathPresentation {
        use salsa::DisplayWithDb;

        let db = self.db();
        let item_path_id: ItemPathId = item_path_id_interface.into();
        ItemPathPresentation::new(
            format!("{}", item_path_id.ident(db).unwrap().data(db)),
            format!("{:?}", item_path_id.item_path(db).display(db)),
        )
    }

    fn calc_var_id_presentations(
        &self,
        var_id: TraceVarId<Self::TraceProtocol>,
    ) -> VarIdPresentation {
        VarIdPresentation::new(format!("{:?}", var_id))
    }

    fn add_extra_var_deps_to_caryatid(
        &self,
        caryatid: <Self::TraceProtocol as IsTraceProtocol>::Caryatid,
        var_deps: &[ItemPathIdInterface],
        page_limit: usize,
    ) -> DevsoulStaticVarResult<Devsoul, <Devsoul::TraceProtocol as IsTraceProtocol>::Caryatid>
    {
        self.runtime
            .with_var_ids(
                caryatid
                    .clone()
                    .var_path_windlasses()
                    .map(|(item_path_id_interface, windlass)| match windlass {
                        Windlass::Specific(var_id)
                        | Windlass::Generic {
                            page_start: var_id, ..
                        } => (item_path_id_interface, var_id),
                    }),
                |locked| {
                    self.add_extra_var_deps_to_caryatid_aux(
                        caryatid.clone(),
                        var_deps
                            .iter()
                            .copied()
                            .filter(|&var_dep| !caryatid.has_var_dep(var_dep)),
                        locked,
                        page_limit,
                    )
                },
            )
            .flatten()
    }
}

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    fn add_extra_var_deps_to_caryatid_aux(
        &self,
        mut caryatid: <Devsoul::TraceProtocol as IsTraceProtocol>::Caryatid,
        mut var_deps: impl Iterator<Item = ItemPathIdInterface>,
        mut locked: SmallVecSet<ItemPathIdInterface, 4>,
        page_limit: usize,
    ) -> DevsoulStaticVarResult<Devsoul, <Devsoul::TraceProtocol as IsTraceProtocol>::Caryatid>
    {
        match var_deps.next() {
            Some(item_path_id_interface) => {
                let page_start = self
                    .runtime
                    .var_default_page_start_aux(item_path_id_interface, &locked)?;
                caryatid.add_new(
                    item_path_id_interface,
                    Windlass::Generic {
                        page_start,
                        followed: Some(page_start),
                        page_limit: Some(page_limit),
                    },
                );
                self.runtime
                    .with_var_id(item_path_id_interface, page_start, &locked, |locked| {
                        self.add_extra_var_deps_to_caryatid_aux(
                            caryatid, var_deps, locked, page_limit,
                        )
                    })
                    .flatten()
            }
            None => Ok(caryatid),
        }
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
