mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDeclarativeSignatureTemplate {
    AssociatedFn(TraitForTypeAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate),
    AssociatedType(TraitForTypeAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedVal(TraitForTypeAssociatedValDeclarativeSignatureTemplate),
}

pub(crate) fn trai_for_ty_associated_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeItemDecl,
) -> DeclarativeSignatureResult<TraitForTypeItemDeclarativeSignatureTemplate> {
    match decl {
        TraitForTypeItemDecl::AssociatedFn(decl) => {
            trai_for_ty_associated_fn_declarative_signature_template(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::MethodFn(decl) => {
            trai_for_ty_method_fn_signature(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::AssociatedType(decl) => {
            trai_for_ty_associated_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TraitForTypeItemDecl::AssociatedVal(decl) => {
            trai_for_ty_associated_val_declarative_signature_template(db, decl).map(Into::into)
        }
    }
}

impl TraitForTypeItemDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedFn(_) => todo!(),
            TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
        }
    }
}
