mod runtime_storage;

pub use husky_standard_devsoul_interface::pedestal::StandardPedestal;

use self::runtime_storage::*;
use husky_devsoul::devsoul::{DevEvalContextLocalKey, IsDevsoul};
use husky_devsoul_interface::{
    item_path::ItemPathIdInterface,
    ki_control_flow::KiControlFlow,
    ki_repr::{KiDomainReprInterface, KiReprInterface},
    IsDevRuntime, IsDevRuntimeDyn,
};
use husky_mono_linktime::MonoLinktime;
use husky_standard_trace_protocol::{caryatid::StandardCaryatid, StandardTraceProtocol};
use husky_trace_protocol::{
    caryatid::IsCaryatid, figure::IsFigure, id::TraceId, protocol::IsTraceProtocol,
    server::ValVisualCache,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct StandardDevsoul<Figure>(PhantomData<Figure>)
where
    Figure: IsFigure<StandardPedestal>;

type LinketImpl = husky_linket_impl::standard::StandardLinketImpl<StandardPedestal>;

impl<Figure> IsDevsoul for StandardDevsoul<Figure>
where
    Figure: IsFigure<StandardPedestal>,
{
    type Pedestal = StandardPedestal;

    type LinketImpl = LinketImpl;

    type Linktime = MonoLinktime<LinketImpl>;

    type RuntimeStorage = StandardDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = StandardTraceProtocol<Figure>;

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
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        let (followed, followed_domain_and_var_deps) = match followed {
            Some((trace_id, ki_repr, ki_domain_repr_interface, var_deps)) => (
                Some((trace_id, ki_repr)),
                Some((ki_domain_repr_interface, var_deps)),
            ),
            None => (None, None),
        };
        // checking that caryatid should at least cover the var deps of the followed
        if let Some((_, followed_var_deps)) = followed_domain_and_var_deps {
            assert!(caryatid.covers(followed_var_deps))
        }
        // throw away unnessary things
        let accompanyings = &accompanyings
            .iter()
            .copied()
            .filter_map(|(trace_id, ki_repr, var_deps)| {
                caryatid.covers(var_deps).then_some((trace_id, ki_repr))
            })
            .collect::<Vec<_>>();
        if caryatid.is_specific() {
            <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<StandardPedestal>>::new_specific(
                    followed ,
                    accompanyings,
                    |ki_repr, visual_synchrotron| {
                        Self::get_ki_visual(
                            ki_repr,
                            runtime,
                            visual_synchrotron,
                            val_visual_cache,
                        )
                    },
                    visual_synchrotron,
                )
        } else {
            todo!()
            //         let pedestals = (0..49).into_iter().filter_map(|index| {
            //                 let pedestal = StandardPedestal::Specific(DeprecatedInputId::from_index(index));
            //                 let Some(ki_domain_repr_interface) = domain else {
            //                     return Some(pedestal)
            //                 };
            //                 match runtime.eval_ki_domain_repr_interface_dyn(
            //                     ki_domain_repr_interface,
            //                 ) {
            //                     KiControlFlow::Continue(_) => Some(pedestal),
            //                     KiControlFlow::LoopContinue => todo!(),
            //                     KiControlFlow::LoopExit(_) => todo!(),
            //                     KiControlFlow::Return(_) => todo!(),
            //                     KiControlFlow::Undefined => todo!(),
            //                     KiControlFlow::Throw(_) => todo!(),
            //                 }
            //             });
            //         <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<StandardPedestal>>::new_generic(
            //             followed,
            //             accompanyings,
            //             pedestals,
            //             |ki_repr, pedestal, visual_synchrotron| {
            //                 Self::get_ki_visual(
            //                     ki_repr,
            //                     runtime,
            //                     visual_synchrotron,
            //                     val_visual_cache,
            //                 )
            //             },
            //             visual_synchrotron,
            //         )
            //     }
        }
    }
}
