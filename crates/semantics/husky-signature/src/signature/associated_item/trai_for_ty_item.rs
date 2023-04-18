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
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDeclarativeSignature {
    AssociatedFn(TraitForTypeAssociatedFnSignature),
    MethodFn(TraitForTypeMethodSignature),
    AssociatedType(TraitForTypeAssociatedTypeDeclarativeSignature),
    AssociatedValue(TraitForTypeAssociatedValueSignature),
}

pub(crate) fn trai_for_ty_associated_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeItemDecl,
) -> DeclarativeSignatureResult<TraitForTypeItemDeclarativeSignature> {
    match decl {
        TraitForTypeItemDecl::AssociatedFunction(decl) => {
            trai_for_ty_associated_form_fn_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::Method(decl) => {
            trai_for_ty_method_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::AssociatedType(decl) => {
            trai_for_ty_associated_ty_declarative_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::AssociatedValue(decl) => {
            trai_for_ty_associated_value_declarative_signature(db, decl).map(Into::into)
        }
    }
}

impl TraitForTypeItemDeclarativeSignature {
    pub fn implicit_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TraitForTypeItemDeclarativeSignature::AssociatedFn(_) => todo!(),
            TraitForTypeItemDeclarativeSignature::MethodFn(_) => todo!(),
            TraitForTypeItemDeclarativeSignature::AssociatedType(_) => todo!(),
            TraitForTypeItemDeclarativeSignature::AssociatedValue(_) => todo!(),
        }
    }
}
