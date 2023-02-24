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
#[salsa::derive_debug_with_db(db = DeclDb)]
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

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            FormDecl::Function(decl) => decl.implicit_parameters(db),
            FormDecl::Feature(decl) => Ok(&[]),
            FormDecl::Morphism(decl) => decl.implicit_parameters(db),
            FormDecl::Value(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            FormDecl::Function(decl) => decl.expr_region(db),
            FormDecl::Feature(decl) => decl.expr_region(db),
            FormDecl::Morphism(decl) => decl.expr_region(db),
            FormDecl::Value(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> FormPath {
        match self {
            FormDecl::Function(decl) => decl.path(db),
            FormDecl::Feature(decl) => decl.path(db),
            FormDecl::Morphism(decl) => decl.path(db),
            FormDecl::Value(decl) => decl.path(db),
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
