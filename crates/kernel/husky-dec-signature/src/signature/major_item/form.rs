pub mod r#const;
pub mod function_ritchie;
pub mod ty_alias;
pub mod val;

use self::function_ritchie::*;
use self::r#const::*;
use self::ty_alias::*;
use self::val::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormDecTemplate {
    Ritchie(MajorFunctionRitchieDecTemplate),
    TypeAlias(TypeAliasDecTemplate),
    Val(MajorValDecTemplate),
    Const(MajorConstDecTemplate),
}

impl MajorFormDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            MajorFormDecTemplate::Ritchie(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::Val(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::TypeAlias(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::Const(slf) => slf.template_parameters(db),
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
        FormSynDecl::Const(decl) => MajorConstDecTemplate::from_decl(db, decl).map(Into::into),
    }
}
