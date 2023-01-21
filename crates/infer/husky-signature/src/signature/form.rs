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

pub(crate) fn form_signature(db: &dyn SignatureDb, decl: FormDecl) -> FormSignature {
    match decl {
        FormDecl::Function(decl) => function_signature(db, decl).into(),
        FormDecl::Feature(decl) => feature_signature(db, decl).into(),
        FormDecl::Morphism(decl) => morphism_signature(db, decl).into(),
        FormDecl::Value(decl) => value_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FormSignature {
    Function(FunctionSignature),
    Feature(FeatureSignature),
    Morphism(MorphismSignature),
    Value(ValueSignature),
}

impl FormSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            FormSignature::Function(decl) => decl.implicit_parameters(db),
            FormSignature::Feature(decl) => &[],
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

impl From<FunctionSignature> for FormSignature {
    fn from(v: FunctionSignature) -> Self {
        Self::Function(v)
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for FormSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            FormSignature::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            FormSignature::Feature(decl) => f
                .debug_tuple("Feature")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            FormSignature::Morphism(decl) => f
                .debug_tuple("Morphism")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            FormSignature::Value(decl) => f
                .debug_tuple("Value")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
