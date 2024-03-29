mod r#fn;
mod gn;
mod ty_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ty_alias::*;
pub use self::val::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveDecTemplate {
    Fn(MajorFnDecTemplate),
    Gn(MajorGnDecTemplate),
    TypeAlias(TypeAliasDecTemplate),
    Ki(MajorValDecTemplate),
}

impl FugitiveDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            FugitiveDecTemplate::Fn(decl) => decl.template_parameters(db),
            FugitiveDecTemplate::Ki(decl) => decl.template_parameters(db),
            FugitiveDecTemplate::Gn(decl) => decl.template_parameters(db),
            FugitiveDecTemplate::TypeAlias(decl) => decl.template_parameters(db),
        }
    }
}

impl HasDecTemplate for FugitivePath {
    type DecTemplate = FugitiveDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        fugitive_syn_dec_template(db, self)
    }
}

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn fugitive_syn_dec_template(
    db: &::salsa::Db,
    path: FugitivePath,
) -> DecSignatureResult<FugitiveDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        FugitiveSynDecl::Fn(decl) => MajorFnDecTemplate::from_decl(db, decl).map(Into::into),
        FugitiveSynDecl::Ki(decl) => MajorValDecTemplate::from_decl(db, decl).map(Into::into),
        FugitiveSynDecl::FunctionGn(decl) => {
            MajorGnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        FugitiveSynDecl::TypeAlias(decl) => {
            TypeAliasDecTemplate::from_decl(db, decl).map(Into::into)
        }
    }
}
