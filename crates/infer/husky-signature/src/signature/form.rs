mod feature;
mod r#fn;
mod gn;
mod type_alias;
mod value;

pub use feature::*;
pub use gn::*;
pub use r#fn::*;
pub use type_alias::*;
pub use value::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
#[enum_class::from_variants]
pub enum FormSignature {
    Fn(FnSignature),
    Feature(FeatureSignature),
    Gn(GnSignature),
    Value(ValueSignature),
}

impl HasSignature for FormPath {
    type Signature = FormSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        self.decl(db)?.signature(db)
    }
}

impl HasSignature for FormDecl {
    type Signature = FormSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        match self {
            FormDecl::Fn(decl) => decl.signature(db).map(Into::into),
            FormDecl::Feature(decl) => decl.signature(db).map(Into::into),
            FormDecl::Gn(decl) => decl.signature(db).map(Into::into),
            FormDecl::Value(decl) => decl.signature(db).map(Into::into),
        }
    }
}

impl FormSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            FormSignature::Fn(decl) => decl.implicit_parameters(db),
            FormSignature::Feature(decl) => decl.implicit_parameters(db),
            FormSignature::Gn(decl) => decl.implicit_parameters(db),
            FormSignature::Value(decl) => decl.implicit_parameters(db),
        }
    }
}
