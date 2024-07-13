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
