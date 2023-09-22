mod parenate_parameter;
mod template_parameter;

pub use self::parenate_parameter::*;
pub use self::template_parameter::*;

use crate::*;
use husky_regional_token::VarianceRegionalToken;
use husky_syn_expr::{
    SpecificParameterObelisk, SynExprRegionData, TemplateParameterObelisk,
    TemplateParameterObeliskData,
};
