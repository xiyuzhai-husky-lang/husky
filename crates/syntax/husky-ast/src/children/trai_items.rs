use parsec::{ParseFrom, ParseFrom2};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitItems {
    children: AstIdxRange,
}

impl TraitItems {
    pub fn children(&self) -> AstIdxRange {
        self.children
    }
}

impl<'a> ParseFrom2<AstParser<'a>> for TraitItems {
    type Error = AstError;

    type Context = Indent;

    fn parse_from_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
        ctx: Indent,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(todo!())
    }
}
