mod runtime_storage;

use self::runtime_storage::*;
use husky_linkage_impl::IsLinkageImpl;
use husky_mono_linktime::MonoLinkTime;
use husky_regular_value::RegularValue;
use husky_task::{ascension::IsDevAscension, IsTask};
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_visual_protocol::IsVisualProtocol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct MlTask<VisualProtocol>
where
    VisualProtocol: IsVisualProtocol,
{
    _marker: PhantomData<VisualProtocol>,
}

impl<VisualProtocol> MlTask<VisualProtocol>
where
    VisualProtocol: IsVisualProtocol,
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<VisualProtocol> IsTask for MlTask<VisualProtocol>
where
    VisualProtocol: IsVisualProtocol + Send,
{
    type DevAscension = MlDevAscension<VisualProtocol>;
}

pub struct MlDevAscension<VisualProtocol>(PhantomData<VisualProtocol>)
where
    VisualProtocol: IsVisualProtocol;

impl<VisualProtocol> IsDevAscension for MlDevAscension<VisualProtocol>
where
    VisualProtocol: IsVisualProtocol,
{
    type Base = DevInput;

    type Linktime = MonoLinkTime;

    type Value = RegularValue;

    type RuntimeStorage = MlDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = MlTraceProtocol<VisualProtocol>;
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MlTraceProtocol<VisualProtocol>(VisualProtocol);

impl<VisualProtocol> IsTraceProtocol for MlTraceProtocol<VisualProtocol>
where
    VisualProtocol: IsVisualProtocol,
{
    type VisualProtocol = VisualProtocol;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum DevInput {
    Train(),
    Val(),
}
