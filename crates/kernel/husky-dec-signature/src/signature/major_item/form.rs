pub mod compterm;
pub mod function_ritchie;
pub mod r#static;
pub mod ty_alias;
pub mod ty_var;
pub mod val;

use self::{
    compterm::*, function_ritchie::*, r#static::*, ty_alias::*, ty_var::TypeVarDecTemplate, val::*,
};
use super::*;
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_syn_decl::decl::major_item::form::FormSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormDecTemplate {
    Ritchie(MajorFunctionRitchieDecTemplate),
    TypeAlias(TypeAliasDecTemplate),
    TypeVar(TypeVarDecTemplate),
    Val(MajorValDecTemplate),
    Compterm(MajorComptermDecTemplate),
    Static(MajorStaticDecTemplate),
}

impl MajorFormDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            MajorFormDecTemplate::Ritchie(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::Val(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::TypeAlias(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::TypeVar(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::Compterm(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::Static(slf) => slf.template_parameters(db),
        }
    }
}

impl HasDecTemplate for MajorFormPath {
    type DecTemplate = MajorFormDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        form_syn_dec_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn form_syn_dec_template(
    db: &::salsa::Db,
    path: MajorFormPath,
) -> DecSignatureResult<MajorFormDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        FormSynDecl::Ritchie(decl) => {
            MajorFunctionRitchieDecTemplate::from_decl(db, decl).map(Into::into)
        }
        FormSynDecl::Val(decl) => MajorValDecTemplate::from_decl(db, decl).map(Into::into),
        FormSynDecl::TypeAlias(decl) => TypeAliasDecTemplate::from_decl(db, decl).map(Into::into),
        FormSynDecl::TypeVar(decl) => TypeVarDecTemplate::from_decl(db, decl).map(Into::into),
        FormSynDecl::Compterm(decl) => {
            MajorComptermDecTemplate::from_decl(db, decl).map(Into::into)
        }
        FormSynDecl::Static(decl) => MajorStaticDecTemplate::from_decl(db, decl).map(Into::into),
    }
}
