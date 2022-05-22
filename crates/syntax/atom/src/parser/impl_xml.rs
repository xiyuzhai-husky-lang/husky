use super::*;
use crate::*;
use entity_route::RangedEntityRoute;
use text::RangedCustomIdentifier;
use word::{CustomIdentifier, IdentPairDict};

// inner ops
impl<'a> AtomParser<'a> {
    pub fn xml_props(mut self) -> AtomResult<Vec<(CustomIdentifier, URange)>> {
        let mut props: Vec<(CustomIdentifier, URange)> = Vec::new();
        while !self.stream.empty() {
            let ranged_ident = get!(self, custom_ident);
            self.push_abs_semantic_token(token::AbsSemanticToken::new(
                SemanticTokenKind::Parameter,
                ranged_ident.range,
            ));
            no_look_pass!(self, "=");
            no_look_pass!(self, "{");
            self.stream.pop_token_slice();
            let mut layer = 1;
            while layer > 0 {
                match self.stream.next() {
                    Some(token) => match token.kind {
                        TokenKind::Special(Special::LCurl) => layer += 1,
                        TokenKind::Special(Special::RCurl) => layer -= 1,
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
            let token_slice = self.stream.pop_token_slice();
            props.push((
                ranged_ident.ident,
                (token_slice.start)..(token_slice.end - 1),
            ))
        }
        Ok(props)
    }
}
