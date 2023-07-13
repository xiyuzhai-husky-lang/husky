mod generic;
mod specific;

pub use self::generic::*;
pub use self::specific::*;

use crate::*;
use husky_expr::{
    ExprRegionData, GenericParameterDecl, GenericParameterDeclPatternVariant, SpecificParameterDecl,
};
use husky_token::VarianceToken;
