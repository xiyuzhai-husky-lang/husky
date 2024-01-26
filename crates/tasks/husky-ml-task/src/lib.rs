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
    val_control_flow::ValControlFlow,
    val_repr::{ValDomainReprInterface, ValReprInterface},
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
        followed: Option<(TraceId, ValReprInterface, ValDomainReprInterface)>,
        accompanyings: &[(TraceId, ValReprInterface)],
        pedestal: Self::Pedestal,
        runtime: &DevRuntime,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        match pedestal {
            MlPedestal::Specific(_) => {
                <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<MlPedestal>>::new_specific(
                    followed.map(|(v0,v1,_)| (v0,v1)),
                    accompanyings,
                    |val_repr, visual_synchrotron| {
                        Self::get_val_visual(
                            val_repr,
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
                        let Some((_,_,val_domain_repr_interface)) = followed else {
                            return Some(pedestal)
                        };
                        match runtime.eval_val_domain_repr_interface_at_pedestal(
                            val_domain_repr_interface,
                            pedestal
                        ) {
                            ValControlFlow::Continue(_) => Some(pedestal),
                            ValControlFlow::LoopContinue => todo!(),
                            ValControlFlow::LoopExit(_) => todo!(),
                            ValControlFlow::Return(_) => todo!(),
                            ValControlFlow::Undefined => todo!(),
                            ValControlFlow::Err(_) => todo!(),
                        }
                    });
                <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure<MlPedestal>>::new_generic(
                    followed,
                    accompanyings,
                    pedestals,
                    |val_repr, pedestal, visual_synchrotron| {
                        Self::get_val_visual(
                            val_repr,
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
