use entity_route::RangedEntityRoute;
use print_utils::epin;
use vm::*;
use word::WordOpr;

use super::*;

impl<'a> AtomParser<'a> {
    pub(super) fn handle_word_opr(&mut self, word_opr: WordOpr, token: &Token) -> AtomResult<()> {
        let word_opr_range = self.stream.pop_text_range();
        match word_opr {
            WordOpr::And | WordOpr::Or => {
                self.stack.push(Atom::new(word_opr_range, word_opr.into()))
            }
            WordOpr::As => {
                let ty = get!(self, ty?);
                let ty_range = self.stream.pop_text_range();
                self.stack.push(Atom::new(
                    word_opr_range.text_range_to(&ty_range),
                    AtomVariant::Suffix(SuffixOpr::AsTy(RangedEntityRoute {
                        route: ty,
                        range: ty_range,
                    })),
                ))
            }
        }
    }
}
