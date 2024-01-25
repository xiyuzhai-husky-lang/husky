use crate::{op::snap::MnistOpSnap, values::input::Input};
use enum_index::{bitset::EnumBitSet, IsEnumIndex};

#[derive(IsEnumIndex, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trace {
    Input,
    Skeleton,
    ImageFromSkeleton,
    OptimalTransport,
}

pub const ALL_TRACES: &[Trace] = &[Trace::Input];

impl Trace {
    pub(crate) fn value<'a>(
        self,
        input: &'a Input,
        op_snap: &'a MnistOpSnap,
    ) -> &'a dyn std::any::Any {
        match self {
            Trace::Input => input,
            Trace::Skeleton => todo!(),
            Trace::ImageFromSkeleton => todo!(),
            Trace::OptimalTransport => todo!(),
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
}
