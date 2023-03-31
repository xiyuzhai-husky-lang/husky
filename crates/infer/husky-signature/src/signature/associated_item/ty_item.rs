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
    AssociatedFn(TypeAssociatedFnSignature),
    MethodFn(TypeMethodSignature),
    AssociatedType(TypeAssociatedTypeSignature),
    AssociatedValue(TypeAssociatedValueSignature),
    Memo(TypeMemoSignature),
}

impl TypeItemSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeItemSignature::AssociatedFn(signature) => signature.implicit_parameters(db),
            TypeItemSignature::MethodFn(_) => todo!(),
            TypeItemSignature::AssociatedType(_) => todo!(),
            TypeItemSignature::AssociatedValue(_) => todo!(),
            TypeItemSignature::Memo(_) => todo!(),
        }
    }
}

impl HasSignature for TypeItemPath {
    type Signature = TypeItemSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<TypeItemSignature> {
        self.decl(db)?.signature(db)
    }
}

impl HasSignature for TypeItemDecl {
    type Signature = TypeItemSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<TypeItemSignature> {
        ty_item_signature_from_decl(db, self)
    }
}

pub(crate) fn ty_item_signature(
    db: &dyn SignatureDb,
    path: TypeItemPath,
) -> SignatureResult<TypeItemSignature> {
    let decl = path.decl(db)?;
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
