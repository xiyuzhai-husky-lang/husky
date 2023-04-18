mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum FormDeclarativeSignature {
    Fn(FnDeclarativeSignature),
    Val(ValDeclarativeSignature),
    Gn(GnSignature),
}

impl HasDeclarativeSignature for FormPath {
    type DeclarativeSignature = FormDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        self.decl(db)?.declarative_signature(db)
    }
}

impl HasDeclarativeSignature for FormDecl {
    type DeclarativeSignature = FormDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        match self {
            FormDecl::Fn(decl) => decl.declarative_signature(db).map(Into::into),
            FormDecl::Val(decl) => decl.declarative_signature(db).map(Into::into),
            FormDecl::Gn(decl) => decl.declarative_signature(db).map(Into::into),
        }
    }
}

impl FormDeclarativeSignature {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            FormDeclarativeSignature::Fn(decl) => decl.implicit_parameters(db),
            FormDeclarativeSignature::Val(decl) => decl.implicit_parameters(db),
            FormDeclarativeSignature::Gn(decl) => decl.implicit_parameters(db),
        }
    }
}
