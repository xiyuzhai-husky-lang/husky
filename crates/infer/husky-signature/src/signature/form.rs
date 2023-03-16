mod feature;
mod form_fn;
mod morphism;
mod type_alias;
mod value;

pub use feature::*;
pub use form_fn::*;
pub use morphism::*;
pub use type_alias::*;
pub use value::*;

use crate::*;

pub(crate) fn form_signature_from_decl(
    db: &dyn SignatureDb,
    decl: FormDecl,
) -> SignatureResult<FormSignature> {
    match decl {
        FormDecl::Function(decl) => form_fn_signature(db, decl).map(Into::into),
        FormDecl::Feature(decl) => feature_signature(db, decl).map(Into::into),
        FormDecl::Morphism(decl) => morphism_signature(db, decl).map(Into::into),
        FormDecl::Value(decl) => value_signature(db, decl).map(Into::into),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
pub enum FormSignature {
    Function(FormFnSignature),
    Feature(FeatureSignature),
    Morphism(MorphismSignature),
    Value(ValueSignature),
}

impl FormSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            FormSignature::Function(decl) => decl.implicit_parameters(db),
            FormSignature::Feature(_decl) => &[],
            FormSignature::Morphism(decl) => decl.implicit_parameters(db),
            FormSignature::Value(decl) => decl.implicit_parameters(db),
        }
    }
}

impl From<ValueSignature> for FormSignature {
    fn from(v: ValueSignature) -> Self {
        Self::Value(v)
    }
}

impl From<MorphismSignature> for FormSignature {
    fn from(v: MorphismSignature) -> Self {
        Self::Morphism(v)
    }
}

impl From<FeatureSignature> for FormSignature {
    fn from(v: FeatureSignature) -> Self {
        Self::Feature(v)
    }
}

impl From<FormFnSignature> for FormSignature {
    fn from(v: FormFnSignature) -> Self {
        Self::Function(v)
    }
}
