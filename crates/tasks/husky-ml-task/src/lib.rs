mod runtime_storage;

use self::runtime_storage::*;
use husky_ml_task_interface::{MlPedestal, DEV_EVAL_CONTEXT};
use husky_mono_linktime::MonoLinktime;

use husky_task::{
    dev_ascension::{DevEvalContextLocalKey, IsDevAscension},
    IsTask,
};
use husky_task_interface::{val_repr::ValReprInterface, IsDevRuntime};
use husky_trace_protocol::{
    figure::IsFigure, id::TraceId, protocol::IsTraceProtocol, server::ValVisualCache,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct MlTask<VisualProtocol>
where
    VisualProtocol: IsFigure,
{
    _marker: PhantomData<VisualProtocol>,
}

impl<VisualProtocol> MlTask<VisualProtocol>
where
    VisualProtocol: IsFigure,
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<VisualProtocol> IsTask for MlTask<VisualProtocol>
where
    VisualProtocol: IsFigure + Send,
{
    type DevAscension = MlDevAscension<VisualProtocol>;
}

pub struct MlDevAscension<VisualProtocol>(PhantomData<VisualProtocol>)
where
    VisualProtocol: IsFigure;

type LinkageImpl = husky_linkage_impl::standard::LinkageImpl<MlPedestal>;

impl<VisualProtocol> IsDevAscension for MlDevAscension<VisualProtocol>
where
    VisualProtocol: IsFigure,
{
    type Pedestal = MlPedestal;

    type LinkageImpl = LinkageImpl;

    type Linktime = MonoLinktime<LinkageImpl>;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = MlTraceProtocol<VisualProtocol>;

    fn dev_eval_context_local_key() -> &'static DevEvalContextLocalKey<LinkageImpl> {
        &DEV_EVAL_CONTEXT
    }

    fn calc_figure<DevRuntime: IsDevRuntime<Self::LinkageImpl>>(
        followed_trace_id_val_repr_pair: Option<(TraceId, ValReprInterface)>,
        accompanying_trace_id_val_repr_pairs: Vec<(TraceId, ValReprInterface)>,
        pedestal: Self::Pedestal,
        runtime: &DevRuntime,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        match pedestal {
            MlPedestal::Specific(_) => {
                let followed_visual =
                    followed_trace_id_val_repr_pair.map(|(trace_id, val_repr)| {
                        (
                            trace_id,
                            Self::get_val_visual(
                                val_repr,
                                pedestal,
                                runtime,
                                visual_synchrotron,
                                val_visual_cache,
                            ),
                        )
                    });
                let accompanying_visuals = accompanying_trace_id_val_repr_pairs
                    .into_iter()
                    .map(|(trace_id, val_repr)| {
                        (
                            trace_id,
                            Self::get_val_visual(
                                val_repr,
                                pedestal,
                                runtime,
                                visual_synchrotron,
                                val_visual_cache,
                            ),
                        )
                    })
                    .collect::<Vec<_>>();
                <<Self::TraceProtocol as IsTraceProtocol>::Figure as IsFigure>::new_specific(
                    followed_visual,
                    accompanying_visuals,
                    visual_synchrotron,
                )
            }
            MlPedestal::Generic => todo!(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MlTraceProtocol<Figure>(PhantomData<Figure>);

impl<Figure> IsTraceProtocol for MlTraceProtocol<Figure>
where
    Figure: IsFigure,
{
    type Pedestal = MlPedestal;

    type Figure = Figure;
}
