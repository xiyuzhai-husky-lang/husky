mod field;
mod index;
mod method;

pub use self::field::*;
pub use self::index::*;
pub use self::method::*;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyDynamicDispatch<S: MemberSignature> {
    indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>,
    signature: S,
}

type FluffyIndirections = SmallVec<[FluffyDynamicDispatchIndirection; 2]>;

pub trait MemberSignature {
    fn expr_ty(
        &self,
        indirections: &[FluffyDynamicDispatchIndirection],
    ) -> FluffyTermResult<FluffyTerm>;
}

impl<S: MemberSignature> FluffyDynamicDispatch<S> {
    pub fn new(signature: S) -> Self {
        Self {
            indirections: smallvec![],
            signature,
        }
    }

    fn merge(&self, mut indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>) -> Self
    where
        S: Clone,
    {
        indirections.extend(self.indirections.iter().copied());
        Self {
            indirections,
            signature: self.signature.clone(),
        }
    }

    pub fn indirections(&self) -> &[FluffyDynamicDispatchIndirection] {
        &self.indirections
    }

    pub fn signature(&self) -> &S {
        &self.signature
    }

    pub fn expr_ty_result(&self) -> FluffyTermResult<FluffyTerm> {
        self.signature.expr_ty(&self.indirections)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyDynamicDispatchIndirection {
    Place(Place),
    Leash,
}
