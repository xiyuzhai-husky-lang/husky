#![feature(try_trait_v2)]
mod state;
#[cfg(test)]
mod tests;

pub use husky_trace_protocol::server::IsTracetime;

use husky_dev_comptime::DevComptimeTarget;
use husky_dev_runtime::{DevRuntime, DevRuntimeConfig};
use husky_devsoul::{
    devsoul::IsDevsoul,
    helpers::{DevsoulCaryatid, DevsoulChart, DevsoulFigure},
};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr::repr::KiRepr;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface};
use husky_linket_impl::eval_context::IsDevRuntime;
use husky_trace::{jar::TraceDb, trace::Trace};
use husky_trace_protocol::{
    caryatid::IsCaryatid,
    id::TraceId,
    protocol::{IsTraceProtocol, TraceBundle},
    server::KiVisualCache,
    stalk::TraceStalk,
    synchrotron::accompany::AccompanyingTraceIdsExceptFollowed,
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
use smallvec::{SmallVec, ToSmallVec};
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

    fn trace_stalk(
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

    fn figure(
        &self,
        followed_trace: Option<Self::Trace>,
        accompanying_trace_ids_expect_followed: &AccompanyingTraceIdsExceptFollowed,
        caryatid: <Self::TraceProtocol as IsTraceProtocol>::Caryatid,
        visual_synchrotron: &mut VisualSynchrotron,
        ki_visual_cache: &mut KiVisualCache<<Self::TraceProtocol as IsTraceProtocol>::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        let db = self.runtime.db();
        let followed = match followed_trace {
            Some(followed_trace) => followed_trace.ki_repr(db).map(|ki_repr| {
                (
                    followed_trace.into(),
                    ki_repr.into(),
                    ki_repr.ki_domain_repr(db).into(),
                    followed_trace.var_deps(db),
                )
            }),
            None => None,
        };
        let accompanyings_except_followed = &accompanying_trace_ids_expect_followed
            .iter()
            .filter_map(|&accompanying_trace_id| {
                let trace: Trace = accompanying_trace_id.into();
                let ki_repr = trace.ki_repr(db)?;
                Some((trace.into(), ki_repr.into(), trace.var_deps(db)))
            })
            .collect::<Vec<_>>();
        self.calc_figure(
            followed,
            accompanyings_except_followed,
            caryatid,
            visual_synchrotron,
            ki_visual_cache,
        )
    }
}

impl<Devsoul> Devtime<Devsoul>
where
    Devsoul: IsDevsoul,
{
    fn calc_figure(
        &self,
        followed: Option<(
            TraceId,
            KiReprInterface,
            KiDomainReprInterface,
            &[ItemPathIdInterface],
        )>,
        accompanyings: &[(TraceId, KiReprInterface, &[ItemPathIdInterface])],
        caryatid: DevsoulCaryatid<Devsoul>,
        visual_synchrotron: &mut VisualSynchrotron,
        ki_visual_cache: &mut KiVisualCache<Devsoul::Pedestal>,
    ) -> DevsoulFigure<Devsoul> {
        let followed = match followed {
            Some((trace_id, ki_repr, ki_domain_repr_interface, var_deps)) => {
                caryatid.covers(var_deps).then_some((trace_id, ki_repr))
            }
            None => None,
        };
        // throw away unnessary things
        let accompanyings = &accompanyings
            .iter()
            .copied()
            .filter_map(|(trace_id, ki_repr, var_deps)| {
                caryatid.covers(var_deps).then_some((trace_id, ki_repr))
            })
            .collect::<Vec<_>>();
        let static_vars = [todo!()];
        let chart: Option<DevsoulChart<Devsoul, CompositeVisual<TraceId>>> =
            self.runtime.with_static_vars(static_vars, |runtime| {
                if let Some((_, followed_ki_repr_interface)) = followed {
                    let followed_ki_repr: KiRepr = followed_ki_repr_interface.into();
                    match runtime.eval_ki_repr(followed_ki_repr) {
                        KiControlFlow::Continue(_) => (),
                        KiControlFlow::LoopContinue => (),
                        KiControlFlow::LoopExit(_) => (),
                        KiControlFlow::Return(_) => (),
                        KiControlFlow::Undefined => return None,
                        KiControlFlow::Throw(_) => (),
                    }
                }
                let mut t = |(trace_id, ki_repr)| {
                    get_ki_visual(runtime, ki_repr, todo!(), ki_visual_cache)
                        .map(|visual| (trace_id, visual))
                };
                Some(CompositeVisual {
                    followed: match followed {
                        Some(followed) => Some(t(followed)?),
                        None => None,
                    },
                    accompanyings: accompanyings.iter().copied().filter_map(t).collect(),
                })
            });
        todo!();
        // if caryatid.is_specific() {
        //     <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<StandardPedestal>>::new_specific(
        //             followed ,
        //             accompanyings,
        //             |ki_repr, visual_synchrotron| {
        //                 Self::get_ki_visual(
        //                     ki_repr,
        //                     runtime,
        //                     visual_synchrotron,
        //                     ki_visual_cache,
        //                 )
        //             },
        //             visual_synchrotron,
        //         )
        // } else {
        //     todo!()
        //     //         let pedestals = (0..49).into_iter().filter_map(|index| {
        //     //                 let pedestal = StandardPedestal::Specific(DeprecatedInputId::from_index(index));
        //     //                 let Some(ki_domain_repr_interface) = domain else {
        //     //                     return Some(pedestal)
        //     //                 };
        //     //                 match runtime.eval_ki_domain_repr_interface_dyn(
        //     //                     ki_domain_repr_interface,
        //     //                 ) {
        //     //                     KiControlFlow::Continue(_) => Some(pedestal),
        //     //                     KiControlFlow::LoopContinue => todo!(),
        //     //                     KiControlFlow::LoopExit(_) => todo!(),
        //     //                     KiControlFlow::Return(_) => todo!(),
        //     //                     KiControlFlow::Undefined => todo!(),
        //     //                     KiControlFlow::Throw(_) => todo!(),
        //     //                 }
        //     //             });
        //     //         <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<StandardPedestal>>::new_generic(
        //     //             followed,
        //     //             accompanyings,
        //     //             pedestals,
        //     //             |ki_repr, pedestal, visual_synchrotron| {
        //     //                 Self::get_ki_visual(
        //     //                     ki_repr,
        //     //                     runtime,
        //     //                     visual_synchrotron,
        //     //                     ki_visual_cache,
        //     //                 )
        //     //             },
        //     //             visual_synchrotron,
        //     //         )
        //     //     }
        // }
        todo!()
    }
}

fn get_ki_visual<Devsoul: IsDevsoul>(
    runtime: &DevRuntime<Devsoul>,
    ki_repr: KiReprInterface,
    visual_synchrotron: &mut VisualSynchrotron,
    ki_visual_cache: &mut KiVisualCache<Devsoul::Pedestal>,
) -> Option<Visual> {
    let pedestal = todo!();
    use husky_value_interface::IsValue;
    match runtime.eval_ki_repr_interface(ki_repr) {
        KiControlFlow::Continue(value) => Some(ki_visual_cache.get_visual(
            ki_repr,
            pedestal,
            || value.visualize(visual_synchrotron),
        )),
        KiControlFlow::LoopContinue => todo!(),
        KiControlFlow::LoopExit(_) => todo!(),
        KiControlFlow::Return(_) => todo!(),
        KiControlFlow::Undefined => None,
        KiControlFlow::Throw(_) => todo!(),
    }
}

pub trait IsDevtime {}

// ad hoc
pub struct RuntimeConfig {}
