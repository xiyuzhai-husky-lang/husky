use common::*;
use vm::{BoxedValue, EvalValue};

// pub enum CachedValue<'eval> {
//     Defined(&'eval dyn AnyValueDyn),
//     Undefined,
// }

// impl<'eval> CachedValue<'eval> {
//     pub(super) fn defined(&self) -> &'eval dyn AnyValueDyn {
//         match self {
//             CachedValue::Defined(v) => *v,
//             CachedValue::Undefined => panic!(),
//         }
//     }
// }
