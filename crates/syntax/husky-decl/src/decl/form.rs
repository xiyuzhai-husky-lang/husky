mod constant;
mod feature;
mod function;
mod morphism;

pub use constant::*;
pub use feature::*;
pub use function::*;
pub use morphism::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FormDecl {
    Function(FunctionDecl),
    Feature(FeatureDecl),
    Morphism(MorphismDecl),
    Const(ConstantDecl),
}

impl FormDecl {}
