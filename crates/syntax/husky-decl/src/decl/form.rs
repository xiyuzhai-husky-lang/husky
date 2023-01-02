mod constant;
mod feature;
mod function;
mod morphism;
mod type_alias;

pub use constant::*;
pub use feature::*;
pub use function::*;
pub use morphism::*;
pub use type_alias::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FormDecl {
    Function(FunctionDecl),
    Feature(FeatureDecl),
    Morphism(MorphismDecl),
    Const(ConstantDecl),
}

impl FormDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FormDecl::Function(decl) => decl.ast_idx(db),
            FormDecl::Feature(decl) => decl.ast_idx(db),
            FormDecl::Morphism(decl) => decl.ast_idx(db),
            FormDecl::Const(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            FormDecl::Function(_) => todo!(),
            FormDecl::Feature(_) => todo!(),
            FormDecl::Morphism(_) => todo!(),
            FormDecl::Const(_) => todo!(),
        }
    }
}

impl From<ConstantDecl> for FormDecl {
    fn from(v: ConstantDecl) -> Self {
        Self::Const(v)
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
