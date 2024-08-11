pub mod image;
pub mod input;
pub mod optimal_transport;
pub mod optimal_transport_average;
pub mod skeleton;

use crate::{op::history::OpTime, MnistDb, *};
use enum_index::{bitset::EnumBitSet, IsEnumIndex};
use husky_ki_repr_interface::KiReprInterface;
use husky_trace_protocol::id::TraceId;
use husky_visual_protocol::visual::Visual;
use mnist::input_id::MnistInputId;

#[derive(IsEnumIndex, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trace {
    Input,
    Skeleton,
    OptimalTransport,
    OptimalTransportAverage,
}

impl From<TraceId> for Trace {
    fn from(id: TraceId) -> Self {
        Self::from_index(id.index())
    }
}

impl Into<TraceId> for Trace {
    fn into(self) -> TraceId {
        TraceId::from_index(self.index())
    }
}

#[test]
fn trace_from_into_trace_id_works() {
    fn t(trace: Trace) {
        let trace_id: TraceId = trace.into();
        let trace1: Trace = trace_id.into();
        assert_eq!(trace, trace1)
    }

    use Trace::*;
    t(Input);
    t(Skeleton);
    t(OptimalTransport);
}

impl From<KiReprInterface> for Trace {
    fn from(id: KiReprInterface) -> Self {
        Self::from_index(id.index())
    }
}

impl Into<KiReprInterface> for Trace {
    fn into(self) -> KiReprInterface {
        KiReprInterface::from_index(self.index())
    }
}

#[test]
fn trace_from_into_ki_repr_interface_works() {
    fn t(trace: Trace) {
        let ki_repr_interface: KiReprInterface = trace.into();
        let trace1: Trace = ki_repr_interface.into();
        assert_eq!(trace, trace1)
    }

    use Trace::*;
    t(Input);
    t(Skeleton);
    t(OptimalTransport);
}

impl Trace {
    pub(crate) fn visual<'a>(
        self,
        db: &'a MnistDb,
        input_id: MnistInputId,
        op_time: OpTime,
    ) -> Visual {
        match self {
            Trace::Input => db.input_visual(input_id),
            Trace::Skeleton => db.op_history(input_id)[op_time].skeleton_visual(),
            Trace::OptimalTransport => db.op_history(input_id)[op_time].optimal_transport_visual(),
            Trace::OptimalTransportAverage => {
                db.op_history(input_id)[op_time].optimal_transport_average_visual()
            }
        }
    }

    pub(crate) fn code(self) -> &'static str {
        match self {
            Trace::Input => "input",
            Trace::Skeleton => "skeleton",
            Trace::OptimalTransport => "optimal transport",
            Trace::OptimalTransportAverage => "optimal transport average",
        }
    }
}

#[derive(Debug)]
pub struct TraceSelection {
    set: EnumBitSet<Trace>,
}

impl TraceSelection {
    pub fn new(traces: impl IntoIterator<Item = Trace>) -> Self {
        Self {
            set: EnumBitSet::new(traces),
        }
    }
}

impl TraceSelection {
    pub fn set_mut(&mut self) -> &mut EnumBitSet<Trace> {
        &mut self.set
    }

    pub(crate) fn set(&self) -> EnumBitSet<Trace> {
        self.set
    }
}
