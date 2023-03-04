mod associated_ty;
mod associated_value;
mod function;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use method::*;

use super::*;

pub(crate) fn ty_as_trai_associated_item_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeAsTraitItemDecl,
) -> RawSignatureResultRef<TypeAsTraitItemRawSignature> {
    match decl {
        TypeAsTraitItemDecl::AssociatedFunction(decl) => {
            ty_as_trai_associated_function_raw_signature(db, decl)
                .as_ref()
                .map(|s| (*s).into())
        }
        TypeAsTraitItemDecl::Method(decl) => ty_as_trai_method_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeAsTraitItemDecl::AssociatedType(decl) => {
            ty_as_trai_associated_ty_raw_signature(db, decl)
                .as_ref()
                .map(|s| (*s).into())
        }
        TypeAsTraitItemDecl::AssociatedValue(decl) => {
            ty_as_trai_associated_value_raw_signature(db, decl)
                .as_ref()
                .map(|s| (*s).into())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb)]
pub enum TypeAsTraitItemRawSignature {
    Function(TypeAsTraitAssociatedFunctionRawSignature),
    Method(TypeAsTraitMethodRawSignature),
    ExternType(TypeAsTraitAssociatedTypeRawSignature),
    Value(TypeAsTraitAssociatedValueRawSignature),
}

impl From<TypeAsTraitAssociatedValueRawSignature> for TypeAsTraitItemRawSignature {
    fn from(v: TypeAsTraitAssociatedValueRawSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAsTraitAssociatedTypeRawSignature> for TypeAsTraitItemRawSignature {
    fn from(v: TypeAsTraitAssociatedTypeRawSignature) -> Self {
        Self::ExternType(v)
    }
}

impl From<TypeAsTraitMethodRawSignature> for TypeAsTraitItemRawSignature {
    fn from(v: TypeAsTraitMethodRawSignature) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAsTraitAssociatedFunctionRawSignature> for TypeAsTraitItemRawSignature {
    fn from(v: TypeAsTraitAssociatedFunctionRawSignature) -> Self {
        Self::Function(v)
    }
}

impl TypeAsTraitItemRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        match self {
            TypeAsTraitItemRawSignature::Function(_) => todo!(),
            TypeAsTraitItemRawSignature::Method(_) => todo!(),
            TypeAsTraitItemRawSignature::ExternType(_) => todo!(),
            TypeAsTraitItemRawSignature::Value(_) => todo!(),
        }
    }
}
