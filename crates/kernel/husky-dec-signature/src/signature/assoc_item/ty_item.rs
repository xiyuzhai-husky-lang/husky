mod assoc_ritchie;
mod assoc_ty;
mod assoc_val;
mod memo_field;
mod method_curry;
mod method_ritchie;

pub use self::assoc_ritchie::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::memo_field::*;
pub use self::method_curry::*;
pub use self::method_ritchie::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDecTemplate {
    AssocFn(TypeAssocFnDecTemplate),
    MethodFn(TypeMethodRitchieDecTemplate),
    AssocType(TypeAssocTypeDecTemplate),
    AssocVal(TypeAssocValDecTemplate),
    MemoizedField(TypeMemoizedFieldDecTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemDecTemplates {
    AssocFn(SmallVecImpl<TypeAssocFnDecTemplate>),
    MethodFn(SmallVecImpl<TypeMethodRitchieDecTemplate>),
    AssocType(SmallVecImpl<TypeAssocTypeDecTemplate>),
    AssocVal(SmallVecImpl<TypeAssocValDecTemplate>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDecTemplate>),
}

impl TypeItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TypeItemDecTemplate::AssocFn(signature) => signature.template_parameters(db),
            TypeItemDecTemplate::MethodFn(signature) => signature.template_parameters(db),
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

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn ty_item_syn_dec_template(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> DecSignatureResult<TypeItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TypeItemSynDecl::AssocFn(decl) => {
            TypeAssocFnDecTemplate::from_decl(db, path, decl).map(Into::into)
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
