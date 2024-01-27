use crate::{
    op::{frame::MnistOpFrame, history::OpTime},
    values::input::Input,
    MnistDb,
};
use enum_index::{bitset::EnumBitSet, IsEnumIndex};
use husky_ml_task_interface::InputId;
use husky_task_interface::val_repr::ValReprInterface;
use husky_trace_protocol::id::TraceId;
use husky_visual_protocol::visual::Visual;

#[derive(IsEnumIndex, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trace {
    Input,
    Skeleton,
    ImageFromSkeleton,
    OptimalTransport,
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
    t(ImageFromSkeleton);
    t(OptimalTransport);
}

impl From<ValReprInterface> for Trace {
    fn from(id: ValReprInterface) -> Self {
        Self::from_index(id.index())
    }
}

impl Into<ValReprInterface> for Trace {
    fn into(self) -> ValReprInterface {
        ValReprInterface::from_index(self.index())
    }
}

#[test]
fn trace_from_into_val_repr_interface_works() {
    fn t(trace: Trace) {
        let val_repr_interface: ValReprInterface = trace.into();
        let trace1: Trace = val_repr_interface.into();
        assert_eq!(trace, trace1)
    }

    use Trace::*;
    t(Input);
    t(Skeleton);
    t(ImageFromSkeleton);
    t(OptimalTransport);
}

pub const ALL_TRACES: &[Trace] = &[Trace::Input];

impl Trace {
    pub(crate) fn visual<'a>(self, db: &'a MnistDb, input_id: InputId, op_time: OpTime) -> Visual {
        match self {
            Trace::Input => db.input_visual(input_id),
            Trace::Skeleton => db.op_history(input_id)[op_time].skeleton_visual(),
            Trace::ImageFromSkeleton => todo!(),
            Trace::OptimalTransport => db.op_history(input_id)[op_time].optimal_transport_visual(),
        }
    }

    pub(crate) fn code(self) -> &'static str {
        match self {
            Trace::Input => "input",
            Trace::Skeleton => "skeleton",
            Trace::ImageFromSkeleton => "image from skeleton",
            Trace::OptimalTransport => "optimal transport",
        }
    }
}

#[derive(Debug)]
pub struct TraceSelection {
    set: EnumBitSet<Trace>,
}

impl Default for TraceSelection {
    fn default() -> Self {
        Self {
            set: Default::default(),
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
