// provides support for determining expression type

mod instance;
mod ontology;

pub use self::instance::*;
pub use self::ontology::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyInstanceMemberDisambiguation<S: MemberSignature> {
    indirections: SmallVec<[FluffyInstanceIndirection; 2]>,
    signature: S,
}

type FluffyIndirections = SmallVec<[FluffyInstanceIndirection; 2]>;

pub trait MemberSignature {
    fn expr_ty(&self, indirections: &[FluffyInstanceIndirection]) -> FluffyTermResult<FluffyTerm>;
}

impl<S: MemberSignature> FluffyInstanceMemberDisambiguation<S> {
    pub fn new(signature: S) -> Self {
        Self {
            indirections: smallvec![],
            signature,
        }
    }

    fn merge(&self, mut indirections: SmallVec<[FluffyInstanceIndirection; 2]>) -> Self
    where
        S: Clone,
    {
        indirections.extend(self.indirections.iter().copied());
        Self {
            indirections,
            signature: self.signature.clone(),
        }
    }

    pub fn indirections(&self) -> &[FluffyInstanceIndirection] {
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
pub enum FluffyInstanceIndirection {
    Place(Place),
    Leash,
}
