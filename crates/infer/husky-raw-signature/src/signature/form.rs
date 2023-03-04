mod feature;
mod function;
mod morphism;
mod type_alias;
mod value;

pub use feature::*;
pub use function::*;
pub use morphism::*;
pub use type_alias::*;
pub use value::*;

use crate::*;
use salsa::DbWithJar;

pub(crate) fn form_raw_signature(
    db: &dyn RawSignatureDb,
    decl: FormDecl,
) -> RawSignatureResultRef<FormRawSignature> {
    match decl {
        FormDecl::Function(decl) => function_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        FormDecl::Feature(decl) => feature_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        FormDecl::Morphism(decl) => morphism_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        FormDecl::Value(decl) => value_raw_signature(db, decl).as_ref().map(|s| (*s).into()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb, jar = RawSignatureJar)]
pub enum FormRawSignature {
    Function(FunctionRawSignature),
    Feature(FeatureRawSignature),
    Morphism(MorphismRawSignature),
    Value(ValueRawSignature),
}

impl FormRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        match self {
            FormRawSignature::Function(decl) => decl.implicit_parameters(db),
            FormRawSignature::Feature(decl) => &[],
            FormRawSignature::Morphism(decl) => decl.implicit_parameters(db),
            FormRawSignature::Value(decl) => decl.implicit_parameters(db),
        }
    }
}

impl From<ValueRawSignature> for FormRawSignature {
    fn from(v: ValueRawSignature) -> Self {
        Self::Value(v)
    }
}

impl From<MorphismRawSignature> for FormRawSignature {
    fn from(v: MorphismRawSignature) -> Self {
        Self::Morphism(v)
    }
}

impl From<FeatureRawSignature> for FormRawSignature {
    fn from(v: FeatureRawSignature) -> Self {
        Self::Feature(v)
    }
}

impl From<FunctionRawSignature> for FormRawSignature {
    fn from(v: FunctionRawSignature) -> Self {
        Self::Function(v)
    }
}
