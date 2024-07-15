mod runtime_storage;

use husky_standard_devsoul_interface::DeprecatedInputId;
pub use husky_standard_devsoul_interface::{pedestal::StandardPedestal, DEV_EVAL_CONTEXT};
use husky_standard_trace_protocol::StandardTraceProtocol;

use self::runtime_storage::*;
use husky_mono_linktime::MonoLinktime;

use husky_devsoul::devsoul::{DevEvalContextLocalKey, IsDevsoul};
use husky_devsoul_interface::{
    ki_control_flow::KiControlFlow,
    ki_repr::{KiDomainReprInterface, KiReprInterface},
    IsDevRuntime, IsDevRuntimeDyn,
};
use husky_trace_protocol::{
    figure::IsFigure, id::TraceId, protocol::IsTraceProtocol, server::ValVisualCache,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct StandardDevsoul<Figure>(PhantomData<Figure>)
where
    Figure: IsFigure<StandardPedestal>;

type LinkageImpl = husky_linkage_impl::standard::StandardLinkageImpl<StandardPedestal>;

impl<Figure> IsDevsoul for StandardDevsoul<Figure>
where
    Figure: IsFigure<StandardPedestal>,
{
    type Pedestal = StandardPedestal;

    type LinkageImpl = LinkageImpl;

    type Linktime = MonoLinktime<LinkageImpl>;

    type RuntimeStorage = StandardDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = StandardTraceProtocol<Figure>;

    fn dev_eval_context_local_key() -> &'static DevEvalContextLocalKey<LinkageImpl> {
        &DEV_EVAL_CONTEXT
    }

    fn calc_figure(
        followed: Option<(TraceId, KiReprInterface, KiDomainReprInterface)>,
        accompanyings: &[(TraceId, KiReprInterface)],
        pedestal: Self::Pedestal,
        runtime: &dyn IsDevRuntimeDyn<Self::LinkageImpl>,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        let (followed, domain) = match followed {
            Some((trace_id, var_repr_intreface, ki_domain_repr_interface)) => (
                Some((trace_id, var_repr_intreface)),
                Some(ki_domain_repr_interface),
            ),
            None => (None, None),
        };
        match pedestal {
            StandardPedestal::Specific(_) => {
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
            }
            StandardPedestal::Generic => {
                let pedestals = (0..49).into_iter().filter_map(|index| {
                        let pedestal = StandardPedestal::Specific(DeprecatedInputId::from_index(index));
                        let Some(ki_domain_repr_interface) = domain else {
                            return Some(pedestal)
                        };
                        match runtime.eval_ki_domain_repr_interface_dyn(
                            ki_domain_repr_interface,
                        ) {
                            KiControlFlow::Continue(_) => Some(pedestal),
                            KiControlFlow::LoopContinue => todo!(),
                            KiControlFlow::LoopExit(_) => todo!(),
                            KiControlFlow::Return(_) => todo!(),
                            KiControlFlow::Undefined => todo!(),
                            KiControlFlow::Throw(_) => todo!(),
                        }
                    });
                <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<StandardPedestal>>::new_generic(
                    followed,
                    accompanyings,
                    pedestals,
                    |ki_repr, pedestal, visual_synchrotron| {
                        Self::get_ki_visual(
                            ki_repr,
                            runtime,
                            visual_synchrotron,
                            val_visual_cache,
                        )
                    },
                    visual_synchrotron,
                )
            }
        }
    }
}
