mod constant;
mod feature;
mod function;
mod method;
mod morphism;
mod type_alias;

pub use constant::*;
pub use feature::*;
pub use function::*;
pub use method::*;
pub use morphism::*;
pub use type_alias::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FormDecl {
    Function(FunctionDecl),
    Method(MethodDecl),
    Feature(FeatureDecl),
    Morphism(MorphismDecl),
    Const(ConstantDecl),
    TypeAlias(TypeAliasDecl),
}

impl FormDecl {}
