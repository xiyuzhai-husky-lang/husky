use crate::{op::snap::MnistOpSnap, values::input::Input, MnistDb};
use enum_index::{bitset::EnumBitSet, IsEnumIndex};
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

impl Into<TraceId> for Trace {
    fn into(self) -> TraceId {
        todo!()
    }
}

impl From<ValReprInterface> for Trace {
    fn from(value: ValReprInterface) -> Self {
        unsafe { std::mem::transmute(std::mem::transmute::<_, u32>(value) as u8) }
    }
}

impl Into<ValReprInterface> for Trace {
    fn into(self) -> ValReprInterface {
        unsafe { std::mem::transmute(self as u32) }
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
    pub(crate) fn visual<'a>(self, db: &'a MnistDb, op_snap: &'a MnistOpSnap) -> Visual {
        // match self {
        //     Trace::Input => db.input(),
        //     Trace::Skeleton => todo!(),
        //     Trace::ImageFromSkeleton => todo!(),
        //     Trace::OptimalTransport => todo!(),
        // }
        todo!()
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
