mod ritchie;
mod ty_alias;
mod val;

pub use self::ritchie::*;
pub use self::ty_alias::*;
pub use self::val::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormDecTemplate {
    Ritchie(MajorRitchieDecTemplate),
    TypeAlias(TypeAliasDecTemplate),
    Ki(MajorValDecTemplate),
}

impl MajorFormDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            MajorFormDecTemplate::Ritchie(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::Ki(slf) => slf.template_parameters(db),
            MajorFormDecTemplate::TypeAlias(slf) => slf.template_parameters(db),
        }
    }
}

impl HasDecTemplate for MajorFormPath {
    type DecTemplate = MajorFormDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        form_syn_dec_template(db, self)
    }
}

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn form_syn_dec_template(
    db: &::salsa::Db,
    path: MajorFormPath,
) -> DecSignatureResult<MajorFormDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        FormSynDecl::Ritchie(decl) => MajorRitchieDecTemplate::from_decl(db, decl).map(Into::into),
        FormSynDecl::Val(decl) => MajorValDecTemplate::from_decl(db, decl).map(Into::into),
        FormSynDecl::TypeAlias(decl) => TypeAliasDecTemplate::from_decl(db, decl).map(Into::into),
    }
}
