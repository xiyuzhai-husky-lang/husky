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
pub enum TypeAsTraitItemSignature {
    AssociatedFn(TypeAsTraitAssociatedFnSignature),
    MethodFn(TypeAsTraitMethodSignature),
    AssociatedType(TypeAsTraitAssociatedTypeSignature),
    AssociatedValue(TypeAsTraitAssociatedValueSignature),
}

pub(crate) fn trai_for_ty_associated_item_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TypeAsTraitItemDecl,
) -> SignatureResult<TypeAsTraitItemSignature> {
    match decl {
        TypeAsTraitItemDecl::AssociatedFunction(decl) => {
            trai_for_ty_associated_form_fn_signature(db, decl).map(Into::into)
        }
        TypeAsTraitItemDecl::Method(decl) => trai_for_ty_method_signature(db, decl).map(Into::into),
        TypeAsTraitItemDecl::AssociatedType(decl) => {
            trai_for_ty_associated_ty_signature(db, decl).map(Into::into)
        }
        TypeAsTraitItemDecl::AssociatedValue(decl) => {
            trai_for_ty_associated_value_signature(db, decl).map(Into::into)
        }
    }
}

impl TypeAsTraitItemSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeAsTraitItemSignature::AssociatedFn(_) => todo!(),
            TypeAsTraitItemSignature::MethodFn(_) => todo!(),
            TypeAsTraitItemSignature::AssociatedType(_) => todo!(),
            TypeAsTraitItemSignature::AssociatedValue(_) => todo!(),
        }
    }
}
