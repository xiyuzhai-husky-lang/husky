mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TraitItemDeclarativeSignatureTemplate {
    AssociatedFn(TraitAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TraitMethodFnSignature),
    AssociatedType(TraitAssociatedTypeDeclarativeSignatureTemplate),
    Val(TraitAssociatedValueSignature),
}

pub(crate) fn trai_associated_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitItemDecl,
) -> DeclarativeSignatureResult<TraitItemDeclarativeSignatureTemplate> {
    match decl {
        TraitItemDecl::AssociatedFunction(decl) => {
            trai_associated_form_fn_declarative_signature(db, decl).map(Into::into)
        }
        TraitItemDecl::Method(decl) => trai_method_fn_signature(db, decl).map(Into::into),
        TraitItemDecl::AssociatedType(decl) => {
            trai_associated_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TraitItemDecl::Value(decl) => {
            trai_associated_val_declarative_signature(db, decl).map(Into::into)
        }
    }
}

impl TraitItemDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TraitItemDeclarativeSignatureTemplate::AssociatedFn(_) => todo!(),
            TraitItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
            TraitItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
            TraitItemDeclarativeSignatureTemplate::Val(_) => todo!(),
        }
    }
}
