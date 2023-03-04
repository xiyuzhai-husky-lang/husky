mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use method::*;

use super::*;

pub(crate) fn trai_associated_item_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TraitItemDecl,
) -> RawSignatureResultRef<TraitItemRawSignature> {
    match decl {
        TraitItemDecl::AssociatedFunction(decl) => trai_associated_function_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TraitItemDecl::Method(decl) => trai_method_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TraitItemDecl::AssociatedType(decl) => trai_associated_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TraitItemDecl::Value(decl) => trai_associated_value_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb)]
pub enum TraitItemRawSignature {
    Function(TraitAssociatedFunctionRawSignature),
    Method(TraitMethodRawSignature),
    ExternType(TraitAssociatedTypeRawSignature),
    Value(TraitAssociatedValueRawSignature),
}

impl TraitItemRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        match self {
            TraitItemRawSignature::Function(_) => todo!(),
            TraitItemRawSignature::Method(_) => todo!(),
            TraitItemRawSignature::ExternType(_) => todo!(),
            TraitItemRawSignature::Value(_) => todo!(),
        }
    }
}

impl From<TraitAssociatedFunctionRawSignature> for TraitItemRawSignature {
    fn from(v: TraitAssociatedFunctionRawSignature) -> Self {
        Self::Function(v)
    }
}

impl From<TraitMethodRawSignature> for TraitItemRawSignature {
    fn from(v: TraitMethodRawSignature) -> Self {
        Self::Method(v)
    }
}

impl From<TraitAssociatedValueRawSignature> for TraitItemRawSignature {
    fn from(v: TraitAssociatedValueRawSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TraitAssociatedTypeRawSignature> for TraitItemRawSignature {
    fn from(v: TraitAssociatedTypeRawSignature) -> Self {
        Self::ExternType(v)
    }
}
