mod field;
mod index;
mod method;
mod utils;

pub use self::field::*;
pub use self::index::*;
pub use self::method::*;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyDynamicDispatch<S: MemberSignature> {
    indirections: FluffyDynamicDispatchIndirections,
    signature: S,
}

type FluffyIndirections = SmallVec<[FluffyDynamicDispatchIndirection; 2]>;

/// members means dynamic associated items, i.e. those accessed through an instance
pub trait MemberSignature {
    fn expr_ty(
        &self,
        indirections: &[FluffyDynamicDispatchIndirection],
    ) -> FluffyTermResult<FluffyTerm>;
}

impl<S: MemberSignature> FluffyDynamicDispatch<S> {
    pub fn new(signature: S) -> Self {
        Self {
            indirections: Default::default(),
            signature,
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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct FluffyDynamicDispatchIndirections {
    indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>,
}

impl FluffyDynamicDispatchIndirections {
    /// only use this for field dispatch
    pub(crate) fn extend(&mut self, others: &[FluffyDynamicDispatchIndirection]) {
        self.indirections.extend(others.iter().copied())
    }

    pub(crate) fn push(&mut self, new: FluffyDynamicDispatchIndirection) {
        self.indirections.push(new)
    }
}

impl std::ops::Deref for FluffyDynamicDispatchIndirections {
    type Target = [FluffyDynamicDispatchIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}
