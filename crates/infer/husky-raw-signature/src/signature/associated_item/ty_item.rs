mod associated_ty;
mod associated_value;
mod function;
mod memo;
mod method;

pub use associated_ty::*;
pub use associated_value::*;
pub use function::*;
pub use memo::*;
pub use method::*;

use crate::*;

pub(crate) fn ty_associated_item_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeItemDecl,
) -> RawSignatureResultRef<TypeItemRawSignature> {
    match decl {
        TypeItemDecl::Function(decl) => ty_associated_function_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::Method(decl) => ty_method_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::ExternType(decl) => ty_associated_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::Value(decl) => ty_associated_value_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeItemDecl::Memo(decl) => ty_memo_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        // ImplDecl::TypeImpl(decl) => ty_im_raw_signature(db, decl).into(),
        // ImplDecl::TypeAsTraitImpl(decl) => ty_as_trai_im_raw_signature(db, decl).into(),
        // TypeDecl::Union(decl) => union_ty_raw_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb)]
pub enum TypeItemRawSignature {
    Function(TypeAssociatedFunctionRawSignature),
    Method(TypeMethodRawSignature),
    ExternType(TypeAssociatedTypeRawSignature),
    Value(TypeAssociatedValueRawSignature),
    Memo(TypeMemoRawSignature),
}

impl From<TypeMemoRawSignature> for TypeItemRawSignature {
    fn from(v: TypeMemoRawSignature) -> Self {
        Self::Memo(v)
    }
}

impl From<TypeAssociatedFunctionRawSignature> for TypeItemRawSignature {
    fn from(v: TypeAssociatedFunctionRawSignature) -> Self {
        Self::Function(v)
    }
}

impl From<TypeAssociatedValueRawSignature> for TypeItemRawSignature {
    fn from(v: TypeAssociatedValueRawSignature) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAssociatedTypeRawSignature> for TypeItemRawSignature {
    fn from(v: TypeAssociatedTypeRawSignature) -> Self {
        Self::ExternType(v)
    }
}

impl From<TypeMethodRawSignature> for TypeItemRawSignature {
    fn from(v: TypeMethodRawSignature) -> Self {
        Self::Method(v)
    }
}

impl TypeItemRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        match self {
            TypeItemRawSignature::Function(_) => todo!(),
            TypeItemRawSignature::Method(_) => todo!(),
            TypeItemRawSignature::ExternType(_) => todo!(),
            TypeItemRawSignature::Value(_) => todo!(),
            TypeItemRawSignature::Memo(_) => todo!(),
        }
    }
}
