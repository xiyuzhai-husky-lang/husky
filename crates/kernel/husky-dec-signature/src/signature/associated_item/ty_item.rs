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
pub enum TypeItemDecTemplate {
    AssociatedFn(TypeAssociatedFnDecTemplate),
    MethodFn(TypeMethodFnDecTemplate),
    AssociatedType(TypeAssociatedTypeDecTemplate),
    AssociatedVal(TypeAssociatedValDecTemplate),
    MemoizedField(TypeMemoizedFieldDecTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDecTemplates {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnDecTemplate>),
    MethodFn(SmallVecImpl<TypeMethodFnDecTemplate>),
    AssociatedType(SmallVecImpl<TypeAssociatedTypeDecTemplate>),
    AssociatedVal(SmallVecImpl<TypeAssociatedValDecTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDecTemplate>),
}

impl TypeItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TypeItemDecTemplate::AssociatedFn(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::MethodFn(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::AssociatedType(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::AssociatedVal(_) => &[],
            TypeItemDecTemplate::MemoizedField(_) => &[],
        }
    }
}

impl HasDecTemplate for TypeItemPath {
    type DecTemplate = TypeItemDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<TypeItemDecTemplate> {
        ty_item_syn_dec_template(db, self)
    }
}

// #[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn ty_item_syn_dec_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> DecSignatureResult<TypeItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TypeItemSynDecl::AssociatedFn(decl) => {
            TypeAssociatedFnDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::MethodFn(decl) => {
            TypeMethodFnDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::AssociatedType(decl) => {
            TypeAssociatedTypeDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::AssociatedVal(decl) => {
            TypeAssociatedValDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::MemoizedField(decl) => {
            TypeMemoizedFieldDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeMethodDecTemplates {
    MethodFn(SmallVecImpl<TypeMethodFnDecTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodFunctionDecTemplate>),
}

pub trait HasTypeMethodDecTemplates: Copy {
    fn ty_method_dec_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> DecSignatureResult<&'a [(Ident, DecSignatureResult<TypeMethodDecTemplates>)]>;

    fn ty_method_dec_templates<'a>(
        self,
        db: &'a ::salsa::Db,
        ident: Ident,
    ) -> DecSignatureResult<Option<&'a TypeMethodDecTemplates>> {
        use vec_like::VecMapGetEntry;
        match self.ty_method_dec_templates_map(db)?.get_entry(ident) {
            Some((_, Ok(templates))) => Ok(Some(templates)),
            Some((_, Err(e))) => Err(*e),
            None => Ok(None),
        }
    }
}
