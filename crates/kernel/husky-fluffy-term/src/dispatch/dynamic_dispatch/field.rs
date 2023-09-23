mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_coword::Ident;
use husky_ethereal_signature::{PropsFieldEtherealSignature, TypeMemoizedFieldEtherealSignature};

#[derive(Debug, PartialEq, Eq)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FluffyFieldDispatch {
    indirections: FluffyDynamicDispatchIndirections,
    ty_path: TypePath,
    signature: FluffyFieldSignature,
}

impl FluffyFieldDispatch {
    fn merge(&self, mut indirections: FluffyDynamicDispatchIndirections) -> Self {
        indirections.extend(&self.indirections);
        Self {
            indirections,
            ty_path: self.ty_path,
            signature: self.signature,
        }
    }

    pub fn indirections(&self) -> &[FluffyDynamicDispatchIndirection] {
        &self.indirections
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }

    pub fn signature(&self) -> FluffyFieldSignature {
        self.signature
    }
}

impl FluffyTerm {
    /// returns None if no such field
    pub fn field_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermMaybeResult<FluffyFieldDispatch> {
        self.field_dispatch_aux(engine, ident, available_traits, Default::default())
    }

    fn field_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: FluffyDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyFieldDispatch> {
        match self.base_resolved(engine) {
            FluffyTermBase::Ethereal(term) => {
                JustOk(ethereal_ty_field_dispatch(engine.db(), term, ident)?.merge(indirections))
            }
            FluffyTermBase::Solid(term) => {
                term.field_dispatch_aux(engine, ident, available_traits, indirections)
            }
            FluffyTermBase::Hollow(term) => todo!(),
        }
    }
}
