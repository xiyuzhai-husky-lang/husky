mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDeclarativeSignatureTemplate {
    AssociatedFn(TypeAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TypeMethodFnDeclarativeSignatureTemplate),
    AssociatedType(TypeAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedVal(TypeAssociatedValDeclarativeSignatureTemplate),
    MemoizedField(TypeMemoizedFieldDeclarativeSignatureTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDeclarativeSignatureTemplates {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnDeclarativeSignatureTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnDeclarativeSignatureTemplate>),
    AssociatedType(SmallVecImpl<TypeAssociatedTypeDeclarativeSignatureTemplate>),
    AssociatedVal(SmallVecImpl<TypeAssociatedValDeclarativeSignatureTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDeclarativeSignatureTemplate>),
}

impl TypeItemDeclarativeSignatureTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TypeItemDeclarativeSignatureTemplate::AssociatedFn(signature) => {
                signature.template_parameters(db)
            }
            TypeItemDeclarativeSignatureTemplate::MethodFn(signature) => {
                signature.template_parameters(db)
            }
            TypeItemDeclarativeSignatureTemplate::AssociatedType(signature) => {
                signature.template_parameters(db)
            }
            TypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => &[],
            TypeItemDeclarativeSignatureTemplate::MemoizedField(_) => &[],
        }
    }
}

impl HasDeclarativeSignatureTemplate for TypeItemPath {
    type DeclarativeSignatureTemplate = TypeItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
        ty_item_syn_declarative_signature_template(db, self)
    }
}

// #[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_item_syn_declarative_signature_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> DeclarativeSignatureResult<TypeItemDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TypeItemSynDecl::AssociatedFn(decl) => {
            TypeAssociatedFnDeclarativeSignatureTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::MethodFn(decl) => {
            TypeMethodFnDeclarativeSignatureTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::AssociatedType(decl) => {
            TypeAssociatedTypeDeclarativeSignatureTemplate::from_decl(db, path, decl)
                .map(Into::into)
        }
        TypeItemSynDecl::AssociatedVal(decl) => {
            TypeAssociatedValDeclarativeSignatureTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::MemoizedField(decl) => {
            TypeMemoizedFieldDeclarativeSignatureTemplate::from_decl(db, path, decl).map(Into::into)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeMethodDeclarativeSignatureTemplates {
    MethodFn(SmallVecImpl<TypeMethodFnDeclarativeSignatureTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionDeclarativeSignatureTemplate>),
}

pub trait HasTypeMethodDeclarativeSignatureTemplates: Copy {
    fn ty_method_declarative_signature_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> DeclarativeSignatureResult<
        &'a [(
            Ident,
            DeclarativeSignatureResult<TypeMethodDeclarativeSignatureTemplates>,
        )],
    >;

    fn ty_method_declarative_signature_templates<'a>(
        self,
        db: &'a ::salsa::Db,
        ident: Ident,
    ) -> DeclarativeSignatureResult<Option<&'a TypeMethodDeclarativeSignatureTemplates>> {
        use vec_like::VecMapGetEntry;
        match self
            .ty_method_declarative_signature_templates_map(db)?
            .get_entry(ident)
        {
            Some((_, Ok(templates))) => Ok(Some(templates)),
            Some((_, Err(e))) => Err(*e),
            None => Ok(None),
        }
    }
}
