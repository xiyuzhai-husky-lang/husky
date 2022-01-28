use common::*;
use semantics::Opn;
use stdx::sync::ARwLock;
use syntax_types::PrimitiveValue;
use vm::{BinaryOpr, Compiled};
use word::CustomIdentifier;

use super::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Feature {
    Input,
    Literal(PrimitiveValue),
    Assert {
        condition: FeatureId,
    },
    Do {
        first: FeatureId,
        then: FeatureId,
    },
    Cached(FeatureId),
    PrimitiveBinaryFunc {
        func: BinaryOpr,
        lopd: FeatureId,
        ropd: FeatureId,
    },
}

// impl<'sess> Feature<'sess> {
//     pub(super) fn new(kind: Feature) -> Self {
//         Self {
//             cached_values: ARwLock::new(HashMap::new()),
//             kind,
//         }
//     }

//     pub(super) fn cache(
//         &self,
//         input_idx: usize,
//         cached_value: CachedValueStorage<'sess>,
//     ) -> CachedValue<'sess> {
//         let value = unsafe { cached_value.value() };
//         self.cached_values
//             .write(|values| should!(values.insert(input_idx, cached_value).is_none()));
//         value
//     }
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct FeatureId(pub(super) usize);
