mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;
mod method_function;

use std::ops::Deref;

use husky_print_utils::p;
use salsa::DebugWithDb;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeItemDeclarativeSignatureTemplate {
    AssociatedFn(TypeAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TypeMethodFnDeclarativeSignatureTemplate),
    AssociatedType(TypeAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedVal(TypeAssociatedValDeclarativeSignatureTemplate),
    MemoizedField(TypeMemoizedFieldDeclarativeSignatureTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeItemDeclarativeSignatureTemplates {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnDeclarativeSignatureTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnDeclarativeSignatureTemplate>),
    AssociatedType(SmallVecImpl<TypeAssociatedTypeDeclarativeSignatureTemplate>),
    AssociatedVal(SmallVecImpl<TypeAssociatedValDeclarativeSignatureTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDeclarativeSignatureTemplate>),
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
            TypeItemDeclarativeSignatureTemplate::MethodFn(signature) => {
                signature.implicit_parameters(db)
            }
            TypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
            TypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
            TypeItemDeclarativeSignatureTemplate::MemoizedField(_) => todo!(),
        }
    }
}

impl HasDeclarativeSignatureTemplate for TypeItemPath {
    type DeclarativeSignatureTemplate = TypeItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
        self.decl(db)?.declarative_signature_template(db)
    }
}

impl HasDeclarativeSignatureTemplate for TypeItemDecl {
    type DeclarativeSignatureTemplate = TypeItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
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
            ty_associated_fn_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeItemDecl::MethodFn(decl) => {
            ty_method_fn_declarative_signature(db, decl).map(Into::into)
        }
        TypeItemDecl::AssociatedType(decl) => {
            ty_associated_ty_declarative_signature_template_from_decl(db, decl).map(Into::into)
        }
        TypeItemDecl::AssociatedVal(decl) => {
            ty_associated_val_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeItemDecl::MemoizedField(decl) => ty_memo_signature(db, decl).map(Into::into),
    }
}

pub trait HasTypeMethodDeclarativeSignatures: Copy {
    fn ty_method_declarative_signature_templates_map<'a>(
        self,
        db: &'a dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &'a IdentPairMap<SmallVec<[TypeMethodDeclarativeSignatureTemplate; 2]>>,
    >;

    fn ty_method_declarative_signature_templates<'a>(
        self,
        db: &'a dyn DeclarativeSignatureDb,
        ident: Ident,
    ) -> DeclarativeSignatureResult<Option<&'a [TypeMethodDeclarativeSignatureTemplate]>> {
        Ok(self
            .ty_method_declarative_signature_templates_map(db)?
            .get_entry(ident)
            .map(|v| v.1.as_ref()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeMethodDeclarativeSignatureTemplate {
    Fn(TypeMethodFnDeclarativeSignatureTemplate),
    Function(TypeMethodFunctionDeclarativeSignatureTemplate),
}

impl HasTypeMethodDeclarativeSignatures for TypePath {
    fn ty_method_declarative_signature_templates_map<'a>(
        self,
        db: &'a dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<
        &'a IdentPairMap<SmallVec<[TypeMethodDeclarativeSignatureTemplate; 2]>>,
    > {
        ty_method_declarative_signature_templates_map(db, self)
            .as_ref()
            .map_err(|e| *e)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar, return_ref)]
pub(crate) fn ty_method_declarative_signature_templates_map(
    db: &dyn DeclarativeSignatureDb,
    ty_path: TypePath,
) -> DeclarativeSignatureResult<IdentPairMap<SmallVec<[TypeMethodDeclarativeSignatureTemplate; 2]>>>
{
    let item_decls_map = ty_path.item_decls_map(db);
    p!(ty_path.debug(db), item_decls_map.debug(db));
    todo!()
}
