use crate::{op::snap::MnistOpSnap, values::input::Input, MnistDb};
use enum_index::{bitset::EnumBitSet, IsEnumIndex};
use husky_visual_protocol::visual::Visual;

#[derive(IsEnumIndex, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trace {
    Input,
    Skeleton,
    ImageFromSkeleton,
    OptimalTransport,
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
