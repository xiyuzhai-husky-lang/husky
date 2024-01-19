mod runtime_storage;

use self::runtime_storage::*;
use husky_ml_task_interface::{MlPedestal, DEV_EVAL_CONTEXT};
use husky_mono_linktime::MonoLinktime;

use husky_task::{
    dev_ascension::{IsDevAscension, LocalDevEvalContext},
    IsTask,
};
use husky_task_interface::val_repr::ValReprInterface;
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_visual_protocol::IsFigure;
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

    fn dev_eval_context_local_key() -> &'static LocalDevEvalContext<LinkageImpl> {
        &DEV_EVAL_CONTEXT
    }

    fn get_figure(
        followed_val_repr: Option<ValReprInterface>,
        accompanying_trace_ids: Vec<ValReprInterface>,
        pedestal: Self::Pedestal,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure {
        todo!()
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
