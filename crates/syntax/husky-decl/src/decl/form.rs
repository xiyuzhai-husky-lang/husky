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
