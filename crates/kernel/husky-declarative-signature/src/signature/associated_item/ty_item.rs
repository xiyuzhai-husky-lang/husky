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
pub enum TypeItemDeclarativeSignatureTemplate {
    AssociatedFn(TypeAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TypeMethodSignature),
    AssociatedType(TypeAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedValue(TypeAssociatedValueSignature),
    Memo(TypeMemoSignature),
}

impl TypeItemDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TypeItemDeclarativeSignatureTemplate::AssociatedFn(signature) => {
                signature.implicit_parameters(db)
            }
            TypeItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
            TypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
            TypeItemDeclarativeSignatureTemplate::AssociatedValue(_) => todo!(),
            TypeItemDeclarativeSignatureTemplate::Memo(_) => todo!(),
        }
    }
}

impl HasDeclarativeSignatureTemplate for TypeItemPath {
    type DeclarativeSignatureTemplate = TypeItemDeclarativeSignatureTemplate;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
        self.decl(db)?.declarative_signature(db)
    }
}

impl HasDeclarativeSignatureTemplate for TypeItemDecl {
    type DeclarativeSignatureTemplate = TypeItemDeclarativeSignatureTemplate;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
        ty_item_declarative_signature_from_decl(db, self)
    }
}

pub(crate) fn ty_item_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    path: TypeItemPath,
) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    ty_item_declarative_signature_from_decl(db, decl)
}

pub(crate) fn ty_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeItemDecl,
) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
    match decl {
        TypeItemDecl::AssociatedFn(decl) => {
            ty_associated_fn_declarative_signature(db, decl).map(Into::into)
        }
        TypeItemDecl::MethodFn(decl) => ty_method_signature(db, decl).map(Into::into),
        TypeItemDecl::AssociatedType(decl) => {
            ty_associated_ty_declarative_signature_template_from_decl(db, decl).map(Into::into)
        }
        TypeItemDecl::AssociatedValue(decl) => {
            ty_associated_val_declarative_signature(db, decl).map(Into::into)
        }
        TypeItemDecl::Memo(decl) => ty_memo_signature(db, decl).map(Into::into),
    }
}
