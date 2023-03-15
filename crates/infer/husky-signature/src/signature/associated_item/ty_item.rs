mod associated_fn;
mod associated_ty;
mod associated_value;
mod memo;
mod method;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_value::*;
pub use memo::*;
pub use method::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
#[enum_class::from_variants]
pub enum TypeItemSignature {
    Function(TypeAssociatedFnSignature),
    Method(TypeMethodSignature),
    ExternType(TypeAssociatedTypeSignature),
    Value(TypeAssociatedValueSignature),
    Memo(TypeMemoSignature),
}

pub(crate) fn ty_item_signature(
    db: &dyn SignatureDb,
    path: TypeItemPath,
) -> SignatureResult<TypeItemSignature> {
    let decl = db.ty_item_decl(path).ok_or(SignatureError::DeclError)?;
    ty_item_signature_from_decl(db, decl)
}

pub(crate) fn ty_item_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TypeItemDecl,
) -> SignatureResult<TypeItemSignature> {
    match decl {
        TypeItemDecl::AssociatedFn(decl) => ty_associated_fn_signature(db, decl).map(Into::into),
        TypeItemDecl::MethodFn(decl) => ty_method_signature(db, decl).map(Into::into),
        TypeItemDecl::AssociatedType(decl) => {
            ty_associated_ty_signature_from_decl(db, decl).map(Into::into)
        }
        TypeItemDecl::AssociatedValue(decl) => {
            ty_associated_value_signature(db, decl).map(Into::into)
        }
        TypeItemDecl::Memo(decl) => ty_memo_signature(db, decl).map(Into::into),
    }
}

impl TypeItemSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeItemSignature::Function(_) => todo!(),
            TypeItemSignature::Method(_) => todo!(),
            TypeItemSignature::ExternType(_) => todo!(),
            TypeItemSignature::Value(_) => todo!(),
            TypeItemSignature::Memo(_) => todo!(),
        }
    }
}
