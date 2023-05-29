mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;
pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use super::*;
use husky_ethereal_signature::{
    HasTypeMemoizedFieldEtherealSignature, HasTypeMemoizedFieldEtherealSignatureTemplates,
    RegularFieldEtherealSignature, TypeMemoizedFieldEtherealSignature,
};
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq)]
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FluffyFieldDisambiguation {
    indirections: SmallVec<[FluffyIndirection; 2]>,
    ty_path: TypePath,
    signature: FieldFluffySignature,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct FieldFluffySignature {
    ty: FluffyTerm,
}

impl FieldFluffySignature {
    pub fn ty(self) -> FluffyTerm {
        // match self {
        //     FieldEtherealSignature::RegularStruct(_) => todo!(),
        // }
        self.ty
    }
}

impl From<RegularFieldEtherealSignature> for FieldFluffySignature {
    fn from(signature: RegularFieldEtherealSignature) -> Self {
        // ad hoc
        FieldFluffySignature {
            ty: signature.ty().into(),
        }
    }
}

impl From<TypeMemoizedFieldEtherealSignature> for FieldFluffySignature {
    fn from(signature: TypeMemoizedFieldEtherealSignature) -> Self {
        Self {
            // ad hoc
            ty: signature.return_ty().into(),
        }
    }
}

impl FluffyFieldDisambiguation {
    fn merge(&self, mut indirections: SmallVec<[FluffyIndirection; 2]>) -> Self {
        indirections.extend(self.indirections.iter().copied());
        Self {
            indirections,
            ty_path: self.ty_path,
            signature: self.signature,
        }
    }

    pub fn indirections(&self) -> &SmallVec<[FluffyIndirection; 2]> {
        &self.indirections
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }

    pub fn signature(&self) -> FieldFluffySignature {
        self.signature
    }
}

impl FluffyTerm {
    /// returns None if no such field
    pub fn field_disambiguation(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
        self.field_disambiguation_aux(engine, ident, available_traits, smallvec![])
    }

    fn field_disambiguation_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
        match self.nested() {
            NestedFluffyTerm::Ethereal(term) => JustOk(
                ethereal_ty_field_disambiguation(engine.db(), term, ident)?.merge(indirections),
            ),
            NestedFluffyTerm::Solid(term) => {
                term.field_disambiguation_aux(engine, ident, available_traits, indirections)
            }
            NestedFluffyTerm::Hollow(term) => todo!(),
        }
    }
}
