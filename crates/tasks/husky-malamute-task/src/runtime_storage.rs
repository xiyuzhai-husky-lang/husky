use std::panic::AssertUnwindSafe;

use crate::DevInput;
use husky_regular_value::RegularValue;
use husky_val::Val;

#[derive(Debug, Default)]
pub struct MlDevRuntimeStorage {
    data: flurry::HashMap<MlDevRuntimeStorageKey, i32>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum MlDevRuntimeStorageKey {
    ModelInternal { val: Val },
    ConstantVal { val: Val },
    NonConstantVal { val: Val, input: DevInput },
}
