mod runtime_storage;

use husky_ml_task_interface::InputId;
pub use husky_ml_task_interface::{pedestal::MlPedestal, DEV_EVAL_CONTEXT};

use self::runtime_storage::*;
use husky_mono_linktime::MonoLinktime;

use husky_task::{
    dev_ascension::{DevEvalContextLocalKey, IsDevAscension},
    IsTask,
};
use husky_task_interface::{
    ki_control_flow::KiControlFlow,
    ki_repr::{KiReprInterface, ValDomainReprInterface},
    IsDevRuntime,
};
use husky_trace_protocol::{
    figure::IsFigure, id::TraceId, protocol::IsTraceProtocol, server::ValVisualCache,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct MlTask<Figure>
where
    Figure: IsFigure<MlPedestal>,
{
    _marker: PhantomData<Figure>,
}

impl<Figure> MlTask<Figure>
where
    Figure: IsFigure<MlPedestal>,
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<Figure> IsTask for MlTask<Figure>
where
    Figure: IsFigure<MlPedestal> + Send,
{
    type DevAscension = MlDevAscension<Figure>;
}

pub struct MlDevAscension<Figure>(PhantomData<Figure>)
where
    Figure: IsFigure<MlPedestal>;

type LinkageImpl = husky_linkage_impl::standard::LinkageImpl<MlPedestal>;

impl<Figure> IsDevAscension for MlDevAscension<Figure>
where
    Figure: IsFigure<MlPedestal>,
{
    type Pedestal = MlPedestal;

    type LinkageImpl = LinkageImpl;

    type Linktime = MonoLinktime<LinkageImpl>;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = MlTraceProtocol<Figure>;

    fn dev_eval_context_local_key() -> &'static DevEvalContextLocalKey<LinkageImpl> {
        &DEV_EVAL_CONTEXT
    }

    fn calc_figure<DevRuntime: IsDevRuntime<Self::LinkageImpl>>(
        followed: Option<(TraceId, KiReprInterface, ValDomainReprInterface)>,
        accompanyings: &[(TraceId, KiReprInterface)],
        pedestal: Self::Pedestal,
        runtime: &DevRuntime,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        let (followed, domain) = match followed {
            Some((trace_id, var_repr_intreface, val_domain_repr_interface)) => (
                Some((trace_id, var_repr_intreface)),
                Some(val_domain_repr_interface),
            ),
            None => (None, None),
        };
        match pedestal {
            MlPedestal::Specific(_) => {
                <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<MlPedestal>>::new_specific(
                    followed ,
                    accompanyings,
                    |ki_repr, visual_synchrotron| {
                        Self::get_val_visual(
                            ki_repr,
                            pedestal,
                            runtime,
                            visual_synchrotron,
                            val_visual_cache,
                        )
                    },
                    visual_synchrotron,
                )
            }
            MlPedestal::Generic => {
                let pedestals = (0..49).into_iter().filter_map(|index| {
                        let pedestal = MlPedestal::Specific(InputId::from_index(index));
                        let Some(val_domain_repr_interface) = domain else {
                            return Some(pedestal)
                        };
                        match runtime.eval_val_domain_repr_interface_at_pedestal(
                            val_domain_repr_interface,
                            pedestal
                        ) {
                            KiControlFlow::Continue(_) => Some(pedestal),
                            KiControlFlow::LoopContinue => todo!(),
                            KiControlFlow::LoopExit(_) => todo!(),
                            KiControlFlow::Return(_) => todo!(),
                            KiControlFlow::Undefined => todo!(),
                            KiControlFlow::Throw(_) => todo!(),
                        }
                    });
                <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<MlPedestal>>::new_generic(
                    followed,
                    accompanyings,
                    pedestals,
                    |ki_repr, pedestal, visual_synchrotron| {
                        Self::get_val_visual(
                            ki_repr,
                            pedestal,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MlTraceProtocol<Figure: IsFigure<MlPedestal>>(PhantomData<Figure>);

impl<Figure: IsFigure<MlPedestal>> Default for MlTraceProtocol<Figure> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Figure> IsTraceProtocol for MlTraceProtocol<Figure>
where
    Figure: IsFigure<MlPedestal>,
{
    type Pedestal = MlPedestal;

    type Figure = Figure;
}
