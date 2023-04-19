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
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
#[enum_class::from_variants]
pub enum FormDeclarativeSignatureTemplate {
    Fn(FnDeclarativeSignatureTemplate),
    Val(ValDeclarativeSignatureTemplate),
    Gn(GnSignature),
}

impl HasDeclarativeSignatureTemplate for FormPath {
    type DeclarativeSignatureTemplate = FormDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        self.decl(db)?.declarative_signature_template(db)
    }
}

impl HasDeclarativeSignatureTemplate for FormDecl {
    type DeclarativeSignatureTemplate = FormDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        match self {
            FormDecl::Fn(decl) => decl.declarative_signature_template(db).map(Into::into),
            FormDecl::Val(decl) => decl.declarative_signature_template(db).map(Into::into),
            FormDecl::Gn(decl) => decl.declarative_signature_template(db).map(Into::into),
        }
    }
}

impl FormDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            FormDeclarativeSignatureTemplate::Fn(decl) => decl.implicit_parameters(db),
            FormDeclarativeSignatureTemplate::Val(decl) => decl.implicit_parameters(db),
            FormDeclarativeSignatureTemplate::Gn(decl) => decl.implicit_parameters(db),
        }
    }
}
