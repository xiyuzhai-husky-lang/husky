use super::*;
use crate::*;
use husky_entity_route_syntax::RangedEntityRoute;
use husky_text::RangedCustomIdentifier;
use word::{CustomIdentifier, IdentPairDict};

// inner ops
impl<'a, 'b> AtomParser<'a, 'b> {
    pub fn xml_props(mut self) -> AtomResult<Vec<(RangedCustomIdentifier, URange)>> {
        let mut props: Vec<(RangedCustomIdentifier, URange)> = Vec::new();
        while !self.token_stream.empty() {
            let ranged_ident = get!(self, custom_ident);
            eat!(self, "=");
            eat!(self, "{");
            let token_start = self.token_stream.token_position();
            let mut layer = 1;
            while layer > 0 {
                match self.token_stream.next() {
                    Some(token) => match token.kind {
                        TokenKind::Special(SpecialToken::LCurl) => layer += 1,
                        TokenKind::Special(SpecialToken::RCurl) => layer -= 1,
                        _ => (),
                    },
                    None => {
                        return err!(
                            format!("expect `= {{<expr>}}` after it"),
                            ranged_ident.range
                        )
                    }
                }
            }
            let token_end = self.token_stream.token_position() - 1;
            props.push((ranged_ident, token_start..token_end))
        }
        Ok(props)
    }
}
