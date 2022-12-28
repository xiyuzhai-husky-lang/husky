mod constant;
mod feature;
mod function;
mod method;
mod morphism;

pub use constant::*;
pub use feature::*;
pub use function::*;
pub use method::*;
pub use morphism::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FormDecl {
    Function(FunctionDecl),
    Method(MethodDecl),
    Feature(FeatureDecl),
    Morphism(MorphismDecl),
    Const(ConstantDecl),
}

impl FormDecl {}
