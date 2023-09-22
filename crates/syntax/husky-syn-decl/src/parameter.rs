mod parenate_parameter;
mod template_parameter;

pub use self::parenate_parameter::*;
pub use self::template_parameter::*;

pub(crate) type CommaRegionalTokens = SmallVec<[CommaRegionalToken; 2]>;

use crate::*;
use husky_opr::Bracket;
use husky_regional_token::*;
use parsec::{
    parse_separated_list, parse_separated_list_expected, StreamParser, TryParseOptionFromStream,
};
