use crate::*;
use serde::{Deserialize, Serialize};
use shifted_unsigned_int::ShiftedU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct MnistInputId(ShiftedU32);

impl MnistInputId {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

impl From<__StaticVarId> for MnistInputId {
    fn from(id: __StaticVarId) -> Self {
        Self::from_index(id.into())
    }
}

impl Into<__StaticVarId> for MnistInputId {
    fn into(self) -> __StaticVarId {
        self.index().into()
    }
}

thread_local! {
    static __INPUT_ID: std::cell::Cell<Option<MnistInputId>> = Default::default();
}

pub(crate) fn input_id() -> MnistInputId {
    __INPUT_ID.get().unwrap()
}

pub(crate) fn with_input_id<R>(input_id: MnistInputId, f: impl Fn() -> R) -> R {
    let old = __INPUT_ID.replace(Some(input_id));
    let r = f();
    __INPUT_ID.set(old);
    r
}

pub(crate) fn set_input_id(input_id: MnistInputId) {
    __INPUT_ID.set(Some(input_id));
}

pub(crate) fn input_ids() -> impl Iterator<Item = MnistInputId> {
    (0..60000).into_iter().map(MnistInputId::from_index)
}
