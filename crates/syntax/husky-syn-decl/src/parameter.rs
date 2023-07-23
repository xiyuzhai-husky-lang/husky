mod explicit;
mod implicit;

pub use self::explicit::*;
pub use self::implicit::*;

pub(crate) type CommaTokens = SmallVec<[CommaToken; 2]>;

use crate::*;
use husky_opr::Bracket;
use husky_token::*;
use parsec::{
    parse_separated_list, parse_separated_list_expected, StreamParser, TryParseOptionFromStream,
};
