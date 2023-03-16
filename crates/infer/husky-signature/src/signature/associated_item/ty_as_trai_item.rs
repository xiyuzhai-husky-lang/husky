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
pub enum TypeAsTraitItemSignature {
    AssociatedFn(TypeAsTraitAssociatedFnSignature),
    MethodFn(TypeAsTraitMethodSignature),
    AssociatedType(TypeAsTraitAssociatedTypeSignature),
    AssociatedValue(TypeAsTraitAssociatedValueSignature),
}

pub(crate) fn ty_as_trai_associated_item_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TypeAsTraitItemDecl,
) -> SignatureResult<TypeAsTraitItemSignature> {
    match decl {
        TypeAsTraitItemDecl::AssociatedFunction(decl) => {
            ty_as_trai_associated_form_fn_signature(db, decl).map(Into::into)
        }
        TypeAsTraitItemDecl::Method(decl) => ty_as_trai_method_signature(db, decl).map(Into::into),
        TypeAsTraitItemDecl::AssociatedType(decl) => {
            ty_as_trai_associated_ty_signature(db, decl).map(Into::into)
        }
        TypeAsTraitItemDecl::AssociatedValue(decl) => {
            ty_as_trai_associated_value_signature(db, decl).map(Into::into)
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
