mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use self::associated_ty::*;
pub use self::associated_value::*;
pub use self::function::*;
pub use self::method::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemSignature {
    AssociatedFn(TraitForTypeAssociatedFnSignature),
    MethodFn(TraitForTypeMethodSignature),
    AssociatedType(TraitForTypeAssociatedTypeSignature),
    AssociatedValue(TraitForTypeAssociatedValueSignature),
}

pub(crate) fn trai_for_ty_associated_item_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TraitForTypeItemDecl,
) -> SignatureResult<TraitForTypeItemSignature> {
    match decl {
        TraitForTypeItemDecl::AssociatedFunction(decl) => {
            trai_for_ty_associated_form_fn_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::Method(decl) => {
            trai_for_ty_method_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::AssociatedType(decl) => {
            trai_for_ty_associated_ty_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::AssociatedValue(decl) => {
            trai_for_ty_associated_value_signature(db, decl).map(Into::into)
        }
    }
}

impl TraitForTypeItemSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TraitForTypeItemSignature::AssociatedFn(_) => todo!(),
            TraitForTypeItemSignature::MethodFn(_) => todo!(),
            TraitForTypeItemSignature::AssociatedType(_) => todo!(),
            TraitForTypeItemSignature::AssociatedValue(_) => todo!(),
        }
    }
}
