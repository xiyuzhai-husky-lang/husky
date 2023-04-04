mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use method::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
#[enum_class::from_variants]
pub enum TraitItemSignature {
    Function(TraitAssociatedFormFnSignature),
    Method(TraitMethodSignature),
    ExternType(TraitAssociatedTypeSignature),
    Value(TraitAssociatedValueSignature),
}

pub(crate) fn trai_associated_item_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TraitItemDecl,
) -> SignatureResult<TraitItemSignature> {
    match decl {
        TraitItemDecl::AssociatedFunction(decl) => {
            trai_associated_form_fn_signature(db, decl).map(Into::into)
        }
        TraitItemDecl::Method(decl) => trai_method_signature(db, decl).map(Into::into),
        TraitItemDecl::AssociatedType(decl) => {
            trai_associated_ty_signature(db, decl).map(Into::into)
        }
        TraitItemDecl::Value(decl) => trai_associated_value_signature(db, decl).map(Into::into),
    }
}

impl TraitItemSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TraitItemSignature::Function(_) => todo!(),
            TraitItemSignature::Method(_) => todo!(),
            TraitItemSignature::ExternType(_) => todo!(),
            TraitItemSignature::Value(_) => todo!(),
        }
    }
}
