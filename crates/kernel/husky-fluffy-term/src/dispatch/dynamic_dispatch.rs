pub mod binary_opr;
mod field;
mod index;
mod method;
mod utils;

pub use self::field::*;
pub use self::index::*;
pub use self::method::*;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct FlyDynamicDispatch<S: MemberSignature> {
    indirections: FlyIndirections,
    signature: S,
}

/// members means dynamic associated items, i.e. those accessed through an instance
pub trait MemberSignature {
    fn expr_ty(&self, self_value_final_place: FlyPlace) -> FlyTermResult<FlyTerm>;
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
    Place(FlyPlace),
    Leash,
}

impl FlyIndirection {
    fn act(self, initial_place: FlyPlace) -> FlyPlace {
        match self {
            FlyIndirection::Place(place) => match place {
                FlyPlace::Const => todo!(),
                FlyPlace::StackPure { location } => todo!(),
                FlyPlace::ImmutableStackOwned { location } => todo!(),
                FlyPlace::MutableStackOwned { location } => todo!(),
                FlyPlace::Transient => todo!(),
                FlyPlace::Ref { guard } => todo!(),
                FlyPlace::RefMut { guard } => todo!(),
                FlyPlace::Leashed => todo!(),
                FlyPlace::Todo => todo!(),
                FlyPlace::EtherealSymbol(_) => todo!(),
            },
            FlyIndirection::Leash => FlyPlace::Leashed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlyIndirections {
    initial_place: FlyPlace,
    indirections: SmallVec<[FlyIndirection; 2]>,
    final_place: FlyPlace,
}

impl FlyIndirections {
    pub(crate) fn new(initial_place: FlyPlace) -> Self {
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

    pub fn initial_place(&self) -> FlyPlace {
        self.initial_place
    }

    pub fn final_place(&self) -> FlyPlace {
        self.final_place
    }
}

impl std::ops::Deref for FlyIndirections {
    type Target = [FlyIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}
