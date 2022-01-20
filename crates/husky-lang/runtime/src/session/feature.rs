use common::*;
use stdx::sync::ARwLock;
use syntax_types::PrimitiveValue;

use super::{value::CachedValue, *};

pub struct Feature<'sess> {
    cached_values: ARwLock<HashMap<usize, CachedValueStorage<'sess>>>,
    pub(super) kind: FeatureKind,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum FeatureKind {
    Literal(PrimitiveValue),
    FunctionCall,
    Binary,
    MembAccess,
    MembCall,
}

impl<'sess> Feature<'sess> {
    pub(super) fn new(kind: FeatureKind) -> Self {
        Self {
            cached_values: ARwLock::new(HashMap::new()),
            kind,
        }
    }

    pub(super) fn cache(
        &self,
        input_idx: usize,
        cached_value: CachedValueStorage<'sess>,
    ) -> CachedValue<'sess> {
        let value = unsafe { cached_value.value() };
        self.cached_values
            .write(|values| should!(values.insert(input_idx, cached_value).is_none()));
        value
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct FeatureId(pub(super) usize);

#[test]
fn move_box() {
    use common::*;

    let a = Box::new(1);
    let pa: *const dyn Any = &*a;
    let b = a;
    let pb: *const dyn Any = &*b;
    should_eq!(pa, pb);
}
