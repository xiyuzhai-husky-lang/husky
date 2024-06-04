mod parenate_parameter;
mod template_parameter;

pub use self::parenate_parameter::*;
pub use self::template_parameter::*;

pub(crate) type CommaRegionalTokens = SmallVec<[CommaRegionalToken; 2]>;

use crate::*;
use husky_regional_token::*;
use husky_syn_expr::{parser::*, syndicates::*};
use parsec::{IsStreamParser, TryParseOptionFromStream};
