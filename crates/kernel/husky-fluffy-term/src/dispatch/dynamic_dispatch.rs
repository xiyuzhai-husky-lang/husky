pub mod binary_opr;
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
    indirections: FluffyIndirections,
    signature: S,
}

/// members means dynamic associated items, i.e. those accessed through an instance
pub trait MemberSignature {
    fn expr_ty(&self, self_value_final_place: FluffyPlace) -> FluffyTermResult<FluffyTerm>;
}

impl<S: MemberSignature> FluffyDynamicDispatch<S> {
    pub fn new(indirections: FluffyIndirections, signature: S) -> Self {
        Self {
            indirections,
            signature,
        }
    }

    pub fn indirections(&self) -> &FluffyIndirections {
        &self.indirections
    }

    pub fn signature(&self) -> &S {
        &self.signature
    }

    pub fn expr_ty_result(&self) -> FluffyTermResult<FluffyTerm> {
        self.signature.expr_ty(self.indirections.final_place)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyIndirection {
    Place(FluffyPlace),
    Leash,
}

impl FluffyIndirection {
    fn act(self, initial_place: FluffyPlace) -> FluffyPlace {
        match self {
            FluffyIndirection::Place(place) => match place {
                FluffyPlace::Const => todo!(),
                FluffyPlace::StackPure { location } => todo!(),
                FluffyPlace::ImmutableStackOwned { location } => todo!(),
                FluffyPlace::MutableStackOwned { location } => todo!(),
                FluffyPlace::Transient => todo!(),
                FluffyPlace::Ref { guard } => todo!(),
                FluffyPlace::RefMut { guard } => todo!(),
                FluffyPlace::Leashed => todo!(),
                FluffyPlace::Todo => todo!(),
                FluffyPlace::EtherealSymbol(_) => todo!(),
            },
            FluffyIndirection::Leash => FluffyPlace::Leashed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyIndirections {
    initial_place: FluffyPlace,
    indirections: SmallVec<[FluffyIndirection; 2]>,
    final_place: FluffyPlace,
}

impl FluffyIndirections {
    pub(crate) fn new(initial_place: FluffyPlace) -> Self {
        Self {
            initial_place,
            indirections: smallvec![],
            final_place: initial_place,
        }
    }

    pub(crate) fn add(&mut self, indirection: FluffyIndirection) {
        self.final_place = indirection.act(self.initial_place);
        self.indirections.push(indirection)
    }

    pub fn initial_place(&self) -> FluffyPlace {
        self.initial_place
    }

    pub fn final_place(&self) -> FluffyPlace {
        self.final_place
    }
}

impl std::ops::Deref for FluffyIndirections {
    type Target = [FluffyIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}
