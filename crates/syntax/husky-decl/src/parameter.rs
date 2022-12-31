use husky_print_utils::p;
use husky_token::LeftAngleBracketToken;
use parsec::ParseContext;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericParameter {}

pub(crate) fn parse_generic_parameters(
    parser: &mut ExprParser,
) -> DeclResult<Vec<GenericParameter>> {
    Ok(
        if let Some(lcurl) = parser.parse::<LeftAngleBracketToken>()? {
            // p!(path.debug(self.db));
            todo!()
        } else {
            vec![]
        },
    )
}
