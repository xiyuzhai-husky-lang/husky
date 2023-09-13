mod generic;
mod specific;

pub use self::generic::*;
pub use self::specific::*;

use crate::*;
use husky_regional_token::VarianceRegionalToken;
use husky_syn_expr::{
    SpecificParameterObelisk, SynExprRegionData, TemplateParameterDeclPatternVariant,
    TemplateParameterObelisk,
};
