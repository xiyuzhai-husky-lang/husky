mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use method::*;

use super::*;

pub(crate) fn ty_as_trai_associated_item_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitItemDecl,
) -> SignatureResultRef<TypeAsTraitItemSignature> {
    match decl {
        TypeAsTraitItemDecl::Function(decl) => ty_as_trai_associated_function_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeAsTraitItemDecl::Method(decl) => ty_as_trai_method_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeAsTraitItemDecl::AlienType(decl) => ty_as_trai_associated_ty_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeAsTraitItemDecl::Value(decl) => ty_as_trai_associated_value_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
pub enum TypeAsTraitItemSignature {
    Function(TypeAsTraitAssociatedFunctionSignature),
    Method(TypeAsTraitMethodSignature),
    AlienType(TypeAsTraitAssociatedTypeSignature),
    Value(TypeAsTraitAssociatedValueSignature),
}

impl From<TypeAsTraitAssociatedValueSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitAssociatedValueSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAsTraitAssociatedTypeSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitAssociatedTypeSignature) -> Self {
        Self::AlienType(v)
    }
}

impl From<TypeAsTraitMethodSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitMethodSignature) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAsTraitAssociatedFunctionSignature> for TypeAsTraitItemSignature {
    fn from(v: TypeAsTraitAssociatedFunctionSignature) -> Self {
        Self::Function(v)
    }
}

impl TypeAsTraitItemSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeAsTraitItemSignature::Function(_) => todo!(),
            TypeAsTraitItemSignature::Method(_) => todo!(),
            TypeAsTraitItemSignature::AlienType(_) => todo!(),
            TypeAsTraitItemSignature::Value(_) => todo!(),
        }
    }
}
