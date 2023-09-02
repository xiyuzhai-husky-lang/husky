mod generic;
mod specific;

pub use self::generic::*;
pub use self::specific::*;

use crate::*;
use husky_syn_expr::{
    SpecificParameterObelisk, SynExprRegionData, TemplateParameterDeclPatternVariant,
    TemplateParameterObelisk,
};
use husky_token::VarianceToken;
