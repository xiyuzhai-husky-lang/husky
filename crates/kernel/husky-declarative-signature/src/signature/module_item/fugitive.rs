mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum FugitiveDeclarativeSignatureTemplate {
    Fn(FnDeclarativeSignatureTemplate),
    Gn(GnDeclarativeSignatureTemplate),
    AliasType(TypeAliasDeclarativeSignatureTemplate),
    Val(ValDeclarativeSignatureTemplate),
}

impl FugitiveDeclarativeSignatureTemplate {
    pub fn generic_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeGenericParameter] {
        match self {
            FugitiveDeclarativeSignatureTemplate::Fn(decl) => decl.generic_parameters(db),
            FugitiveDeclarativeSignatureTemplate::Val(decl) => decl.generic_parameters(db),
            FugitiveDeclarativeSignatureTemplate::Gn(decl) => decl.generic_parameters(db),
            FugitiveDeclarativeSignatureTemplate::AliasType(_) => todo!(),
        }
    }
}

impl HasDeclarativeSignatureTemplate for FugitivePath {
    type DeclarativeSignatureTemplate = FugitiveDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        fugitive_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn fugitive_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: FugitivePath,
) -> DeclarativeSignatureResult<FugitiveDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    match decl {
        FugitiveDecl::Fn(decl) => decl.declarative_signature_template(db).map(Into::into),
        FugitiveDecl::Val(decl) => decl.declarative_signature_template(db).map(Into::into),
        FugitiveDecl::Gn(decl) => decl.declarative_signature_template(db).map(Into::into),
    }
}
