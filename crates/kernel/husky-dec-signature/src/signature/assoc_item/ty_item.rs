pub mod assoc_ritchie;
pub mod assoc_ty;
pub mod assoc_val;
pub mod memo;
pub mod method_curry;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use self::assoc_ty::*;
use self::assoc_val::*;
use self::memo::*;
use self::method_curry::*;
use self::method_ritchie::*;
use super::*;
use crate::signature::impl_block::ty_impl_block::TypeImplBlockDecTemplate;
use husky_entity_path::path::assoc_item::ty_item::TypeItemPath;
use husky_syn_decl::decl::assoc_item::ty_item::TypeItemSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDecTemplate {
    AssocRitchie(TypeAssocRitchieDecTemplate),
    MethodRitchie(TypeMethodRitchieDecTemplate),
    AssocType(TypeAssocTypeDecTemplate),
    AssocVal(TypeAssocValDecTemplate),
    MemoizedField(TypeMemoizedFieldDecTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDecTemplates {
    AssocRitchie(SmallVecImpl<TypeAssocRitchieDecTemplate>),
    MethodFn(SmallVecImpl<TypeMethodRitchieDecTemplate>),
    AssocType(SmallVecImpl<TypeAssocTypeDecTemplate>),
    AssocVal(SmallVecImpl<TypeAssocValDecTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDecTemplate>),
}

impl TypeItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TypeItemDecTemplate::AssocRitchie(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::MethodRitchie(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::AssocType(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::AssocVal(_) => &[],
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

#[salsa::tracked]
pub(crate) fn ty_item_syn_dec_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> DecSignatureResult<TypeItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TypeItemSynDecl::AssocRitchie(decl) => {
            TypeAssocRitchieDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::MethodFn(decl) => {
            TypeMethodRitchieDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::AssocType(decl) => {
            TypeAssocTypeDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::AssocVal(decl) => {
            TypeAssocValDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TypeItemSynDecl::MemoizedField(decl) => {
            TypeMemoizedFieldDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeMethodDecTemplates {
    MethodFn(SmallVecImpl<TypeMethodRitchieDecTemplate>),
    MethodFunction(SmallVecImpl<TypeMethodCurryDecTemplate>),
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
