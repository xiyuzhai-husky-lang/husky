pub mod binary_opr;
mod field;
mod index;
mod method;
mod utils;

pub use self::field::*;
pub use self::index::*;
pub use self::method::*;

use self::quary::FlyQuary;
use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct FlyDynamicDispatch<S: MemberSignature> {
    indirections: FlyIndirections,
    signature: S,
}

/// members means dynamic associated items, i.e. those accessed through an instance
pub trait MemberSignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm>;
}

impl<S: MemberSignature> FlyDynamicDispatch<S> {
    pub fn new(indirections: FlyIndirections, signature: S) -> Self {
        Self {
            indirections,
            signature,
        }
    }

    pub fn indirections(&self) -> &FlyIndirections {
        &self.indirections
    }

    pub fn signature(&self) -> &S {
        &self.signature
    }

    pub fn expr_ty_result(&self) -> FlyTermResult<FlyTerm> {
        self.signature.expr_ty(self.indirections.final_place)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FlyIndirection {
    QualifiedPlace(FlyQuary),
    Leash,
}

impl FlyIndirection {
    fn act(self, initial_place: FlyQuary) -> FlyQuary {
        match self {
            FlyIndirection::QualifiedPlace(qualified_place) => match qualified_place {
                FlyQuary::Const => todo!(),
                FlyQuary::StackPure { .. } => todo!(),
                FlyQuary::ImmutableOnStack { .. } => todo!(),
                FlyQuary::MutableOnStack { .. } => todo!(),
                FlyQuary::Transient => todo!(),
                FlyQuary::Ref { guard } => todo!(),
                FlyQuary::RefMut { .. } => todo!(),
                FlyQuary::Leashed { .. } => todo!(),
                FlyQuary::Todo => todo!(),
                FlyQuary::EtherealSymbol(_) => todo!(),
            },
            FlyIndirection::Leash => FlyQuary::Leashed { place_idx: None },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlyIndirections {
    initial_place: FlyQuary,
    indirections: SmallVec<[FlyIndirection; 2]>,
    final_place: FlyQuary,
}

impl FlyIndirections {
    pub(crate) fn new(initial_place: FlyQuary) -> Self {
        Self {
            initial_place,
            indirections: smallvec![],
            final_place: initial_place,
        }
    }

    pub(crate) fn add(&mut self, indirection: FlyIndirection) {
        self.final_place = indirection.act(self.initial_place);
        self.indirections.push(indirection)
    }

    pub fn initial_place(&self) -> FlyQuary {
        self.initial_place
    }

    pub fn final_place(&self) -> FlyQuary {
        self.final_place
    }
}

impl std::ops::Deref for FlyIndirections {
    type Target = [FlyIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}
