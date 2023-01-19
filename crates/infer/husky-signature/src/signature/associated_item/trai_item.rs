mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use method::*;

use super::*;

pub(crate) fn trai_associated_item_signature(
    db: &dyn SignatureDb,
    decl: TraitItemDecl,
) -> TraitItemSignature {
    match decl {
        TraitItemDecl::Function(decl) => trai_associated_function_signature(db, decl).into(),
        TraitItemDecl::Method(decl) => trai_method_signature(db, decl).into(),
        TraitItemDecl::AlienType(decl) => trai_associated_ty_signature(db, decl).into(),
        TraitItemDecl::Value(decl) => trai_associated_value_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TraitItemSignature {
    Function(TraitAssociatedFunctionSignature),
    Method(TraitMethodSignature),
    AlienType(TraitAssociatedTypeSignature),
    Value(TraitAssociatedValueSignature),
}

impl TraitItemSignature {
    pub fn term_sheet<'a>(self, db: &'a dyn SignatureDb) -> &'a SignatureTermSheet {
        match self {
            TraitItemSignature::Function(signature) => signature.term_sheet(db),
            TraitItemSignature::Method(signature) => signature.term_sheet(db),
            TraitItemSignature::AlienType(signature) => signature.term_sheet(db),
            TraitItemSignature::Value(signature) => signature.term_sheet(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TraitItemSignature::Function(_) => todo!(),
            TraitItemSignature::Method(_) => todo!(),
            TraitItemSignature::AlienType(_) => todo!(),
            TraitItemSignature::Value(_) => todo!(),
        }
    }
}

impl From<TraitAssociatedFunctionSignature> for TraitItemSignature {
    fn from(v: TraitAssociatedFunctionSignature) -> Self {
        Self::Function(v)
    }
}

impl From<TraitMethodSignature> for TraitItemSignature {
    fn from(v: TraitMethodSignature) -> Self {
        Self::Method(v)
    }
}

impl From<TraitAssociatedValueSignature> for TraitItemSignature {
    fn from(v: TraitAssociatedValueSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TraitAssociatedTypeSignature> for TraitItemSignature {
    fn from(v: TraitAssociatedTypeSignature) -> Self {
        Self::AlienType(v)
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for TraitItemSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
