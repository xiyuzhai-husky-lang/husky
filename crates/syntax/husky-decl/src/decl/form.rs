mod feature;
mod function;
mod morphism;
mod type_alias;
mod value;

pub use feature::*;
pub use function::*;
pub use morphism::*;
use salsa::DbWithJar;
pub use type_alias::*;
pub use value::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FormDecl {
    Function(FunctionDecl),
    Feature(FeatureDecl),
    Morphism(MorphismDecl),
    Value(ValueDecl),
}

impl FormDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FormDecl::Function(decl) => decl.ast_idx(db),
            FormDecl::Feature(decl) => decl.ast_idx(db),
            FormDecl::Morphism(decl) => decl.ast_idx(db),
            FormDecl::Value(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            FormDecl::Function(decl) => decl.implicit_parameters(db),
            FormDecl::Feature(decl) => &[],
            FormDecl::Morphism(decl) => decl.implicit_parameters(db),
            FormDecl::Value(decl) => decl.implicit_parameters(db),
        }
    }
}

impl From<ValueDecl> for FormDecl {
    fn from(v: ValueDecl) -> Self {
        Self::Value(v)
    }
}

impl From<MorphismDecl> for FormDecl {
    fn from(v: MorphismDecl) -> Self {
        Self::Morphism(v)
    }
}

impl From<FeatureDecl> for FormDecl {
    fn from(v: FeatureDecl) -> Self {
        Self::Feature(v)
    }
}

impl From<FunctionDecl> for FormDecl {
    fn from(v: FunctionDecl) -> Self {
        Self::Function(v)
    }
}

impl FormDecl {}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for FormDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            FormDecl::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            FormDecl::Feature(decl) => f
                .debug_tuple("Feature")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            FormDecl::Morphism(decl) => f
                .debug_tuple("Morphism")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            FormDecl::Value(decl) => f
                .debug_tuple("Value")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
