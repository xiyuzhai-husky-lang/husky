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
    Val(ValDeclarativeSignatureTemplate),
    Gn(GnDeclarativeSignatureTemplate),
    TypeAlias(TypeAliasDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for FugitivePath {
    type DeclarativeSignatureTemplate = FugitiveDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        self.decl(db)?.declarative_signature_template(db)
    }
}

impl HasDeclarativeSignatureTemplate for FugitiveDecl {
    type DeclarativeSignatureTemplate = FugitiveDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        match self {
            FugitiveDecl::Fn(decl) => decl.declarative_signature_template(db).map(Into::into),
            FugitiveDecl::Val(decl) => decl.declarative_signature_template(db).map(Into::into),
            FugitiveDecl::Gn(decl) => decl.declarative_signature_template(db).map(Into::into),
        }
    }
}

impl FugitiveDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            FugitiveDeclarativeSignatureTemplate::Fn(decl) => decl.implicit_parameters(db),
            FugitiveDeclarativeSignatureTemplate::Val(decl) => decl.implicit_parameters(db),
            FugitiveDeclarativeSignatureTemplate::Gn(decl) => decl.implicit_parameters(db),
            FugitiveDeclarativeSignatureTemplate::TypeAlias(_) => todo!(),
        }
    }
}
