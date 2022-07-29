use husky_entity_route::RangedEntityRoute;
use husky_print_utils::epin;
use husky_text::TextPosition;
use husky_word::WordOpr;
use vm::*;

use super::*;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub(super) fn handle_word_opr(
        &mut self,
        word_opr: WordOpr,
        text_start: TextPosition,
    ) -> AtomResult<()> {
        match word_opr {
            WordOpr::And | WordOpr::Or => {
                let kind = word_opr.into();
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    kind,
                ))
            }
            WordOpr::As => {
                let ty = get!(self, ranged_ty?);
                {
                    let kind = AtomVariant::Suffix(SuffixOpr::AsTy(ty));
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        kind,
                    ))
                }
            }
            WordOpr::Be => {
                let pattern = self.parse_pattern();
                todo!()
            }
        }
    }
}
