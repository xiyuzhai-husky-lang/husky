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
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FormDecl {
    Fn(FnDecl),
    Feature(FeatureDecl),
    Gn(GnDecl),
    Value(ValueDecl),
}

impl FormDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FormDecl::Fn(decl) => decl.ast_idx(db),
            FormDecl::Feature(decl) => decl.ast_idx(db),
            FormDecl::Gn(decl) => decl.ast_idx(db),
            FormDecl::Value(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            FormDecl::Fn(decl) => decl.implicit_parameters(db),
            FormDecl::Feature(_decl) => Ok(&[]),
            FormDecl::Gn(decl) => decl.implicit_parameters(db),
            FormDecl::Value(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            FormDecl::Fn(decl) => decl.expr_region(db),
            FormDecl::Feature(decl) => decl.expr_region(db),
            FormDecl::Gn(decl) => decl.expr_region(db),
            FormDecl::Value(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> FormPath {
        match self {
            FormDecl::Fn(decl) => decl.path(db),
            FormDecl::Feature(decl) => decl.path(db),
            FormDecl::Gn(decl) => decl.path(db),
            FormDecl::Value(decl) => decl.path(db),
        }
    }
}
