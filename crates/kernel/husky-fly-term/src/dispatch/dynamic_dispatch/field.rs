mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;

use super::*;
use husky_coword::Ident;

#[deprecated(note = "use FlyMemberDynamicDispatch instantiation instead")]
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct FlyFieldDyanmicDispatch {
    indirections: FlyIndirections,
    ty_path: TypePath,
    signature: FlyFieldSignature,
}

impl FlyFieldDyanmicDispatch {
    pub fn indirections(&self) -> &FlyIndirections {
        &self.indirections
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }

    pub fn signature(&self) -> &FlyFieldSignature {
        &self.signature
    }

    pub fn expr_ty(&self) -> FlyTerm {
        self.signature
            .return_ty()
            .with_quary(self.indirections.final_place)
    }
}

impl FlyTerm {
    /// returns None if no such field
    pub fn field_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
        self.field_dispatch_aux(
            engine,
            ident,
            available_traits,
            FlyIndirections::new(self.initial_place()),
        )
    }

    fn field_dispatch_aux(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
        match self.base_resolved(engine) {
            FlyTermBase::Eth(term) => {
                ethereal_ty_field_dispatch(engine.db(), term, ident, indirections)
            }
            FlyTermBase::Sol(term) => {
                term.field_dispatch_aux(engine, ident, available_traits, indirections)
            }
            FlyTermBase::Hol(term) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }
}
