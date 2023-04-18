mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_val::*;
pub use memoized_field::*;
pub use method_fn::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeItemDeclarativeSignature {
    AssociatedFn(TypeAssociatedFnDeclarativeSignature),
    MethodFn(TypeMethodSignature),
    AssociatedType(TypeAssociatedTypeDeclarativeSignature),
    AssociatedValue(TypeAssociatedValueSignature),
    Memo(TypeMemoSignature),
}

impl TypeItemDeclarativeSignature {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TypeItemDeclarativeSignature::AssociatedFn(signature) => {
                signature.implicit_parameters(db)
            }
            TypeItemDeclarativeSignature::MethodFn(_) => todo!(),
            TypeItemDeclarativeSignature::AssociatedType(_) => todo!(),
            TypeItemDeclarativeSignature::AssociatedValue(_) => todo!(),
            TypeItemDeclarativeSignature::Memo(_) => todo!(),
        }
    }
}

impl HasDeclarativeSignature for TypeItemPath {
    type DeclarativeSignature = TypeItemDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignature> {
        self.decl(db)?.declarative_signature(db)
    }
}

impl HasDeclarativeSignature for TypeItemDecl {
    type DeclarativeSignature = TypeItemDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignature> {
        ty_item_declarative_signature_from_decl(db, self)
    }
}

pub(crate) fn ty_item_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    path: TypeItemPath,
) -> DeclarativeSignatureResult<TypeItemDeclarativeSignature> {
    let decl = path.decl(db)?;
    ty_item_declarative_signature_from_decl(db, decl)
}

pub(crate) fn ty_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeItemDecl,
) -> DeclarativeSignatureResult<TypeItemDeclarativeSignature> {
    match decl {
        TypeItemDecl::AssociatedFn(decl) => {
            ty_associated_fn_declarative_signature(db, decl).map(Into::into)
        }
        TypeItemDecl::MethodFn(decl) => ty_method_signature(db, decl).map(Into::into),
        TypeItemDecl::AssociatedType(decl) => {
            ty_associated_ty_declarative_signature_from_decl(db, decl).map(Into::into)
        }
        TypeItemDecl::AssociatedValue(decl) => {
            ty_associated_val_declarative_signature(db, decl).map(Into::into)
        }
        TypeItemDecl::Memo(decl) => ty_memo_signature(db, decl).map(Into::into),
    }
}
