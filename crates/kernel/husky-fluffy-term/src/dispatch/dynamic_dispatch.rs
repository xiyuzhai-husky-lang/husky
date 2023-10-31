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
    indirections: FluffyTermDynamicDispatchIndirections,
    signature: S,
}

/// members means dynamic associated items, i.e. those accessed through an instance
pub trait MemberSignature {
    fn expr_ty(&self) -> FluffyTermResult<FluffyTerm>;
}

impl<S: MemberSignature> FluffyDynamicDispatch<S> {
    pub fn new(indirections: FluffyTermDynamicDispatchIndirections, signature: S) -> Self {
        Self {
            indirections,
            signature,
        }
    }

    pub fn indirections(&self) -> &[FluffyTermDynamicDispatchIndirection] {
        &self.indirections
    }

    pub fn signature(&self) -> &S {
        &self.signature
    }

    pub fn expr_ty_result(&self) -> FluffyTermResult<FluffyTerm> {
        self.signature.expr_ty()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyTermDynamicDispatchIndirection {
    Place(Place),
    Leash,
}

impl FluffyTermDynamicDispatchIndirection {
    fn act(self, initial_place: Place) -> Place {
        match self {
            FluffyTermDynamicDispatchIndirection::Place(place) => match place {
                Place::Const => todo!(),
                Place::StackPure { location } => todo!(),
                Place::ImmutableStackOwned { location } => todo!(),
                Place::MutableStackOwned { location } => todo!(),
                Place::Transient => todo!(),
                Place::Ref { guard } => todo!(),
                Place::RefMut { guard } => todo!(),
                Place::Leashed => todo!(),
                Place::Todo => todo!(),
            },
            FluffyTermDynamicDispatchIndirection::Leash => Place::Leashed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTermDynamicDispatchIndirections {
    initial_place: Place,
    indirections: SmallVec<[FluffyTermDynamicDispatchIndirection; 2]>,
    final_place: Place,
}

impl FluffyTermDynamicDispatchIndirections {
    pub(crate) fn new(initial_place: Place) -> Self {
        Self {
            initial_place,
            indirections: smallvec![],
            final_place: initial_place,
        }
    }

    pub(crate) fn add(&mut self, indirection: FluffyTermDynamicDispatchIndirection) {
        self.final_place = indirection.act(self.initial_place);
        self.indirections.push(indirection)
    }

    pub(crate) fn final_place(&self) -> Place {
        self.final_place
    }
}

impl std::ops::Deref for FluffyTermDynamicDispatchIndirections {
    type Target = [FluffyTermDynamicDispatchIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}
