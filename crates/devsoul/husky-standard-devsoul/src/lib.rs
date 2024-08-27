mod runtime_storage;

use self::runtime_storage::*;
use husky_devsoul::devsoul::{DevEvalContextLocalKey, IsDevsoul};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface};
use husky_linket_impl::eval_context::IsDevRuntimeDyn;
use husky_mono_linktime::MonoLinktime;
use husky_standard_linket_impl::pedestal::StandardPedestal;
use husky_standard_trace_protocol::{caryatid::StandardCaryatid, StandardTraceProtocol};
use husky_trace_protocol::{
    caryatid::IsCaryatid, figure::IsFigure, id::TraceId, protocol::IsTraceProtocol,
    server::KiVisualCache,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct StandardDevsoul;

type LinketImpl = husky_standard_linket_impl::StandardLinketImpl;

impl IsDevsoul for StandardDevsoul {
    type Pedestal = StandardPedestal;

    type LinketImpl = LinketImpl;

    type Linktime = MonoLinktime<LinketImpl>;

    type RuntimeStorage = StandardDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = StandardTraceProtocol;

    fn calc_figure(
        followed: Option<(
            TraceId,
            KiReprInterface,
            KiDomainReprInterface,
            &[ItemPathIdInterface],
        )>,
        accompanyings: &[(TraceId, KiReprInterface, &[ItemPathIdInterface])],
        caryatid: StandardCaryatid,
        runtime: &dyn IsDevRuntimeDyn<Self::LinketImpl>,
        visual_synchrotron: &mut VisualSynchrotron,
        ki_visual_cache: &mut KiVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
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
    }
}
