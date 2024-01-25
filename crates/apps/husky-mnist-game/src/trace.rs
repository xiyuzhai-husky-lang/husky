use crate::{op::snap::MnistOpSnap, values::input::Input};
use enum_index::IsEnumIndex;

#[derive(IsEnumIndex, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trace {
    Input,
    Other,
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
            Trace::Other => todo!(),
        }
    }
}

pub struct TraceSelection {
    flags: Vec<bool>,
}

impl Default for TraceSelection {
    fn default() -> Self {
        let flags = (0..(<Trace as IsEnumIndex>::N))
            .into_iter()
            .map(|_| false)
            .collect();
        Self { flags }
    }
}
