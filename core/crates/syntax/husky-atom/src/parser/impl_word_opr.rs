use husky_entity_route::RangedEntityRoute;
use husky_print_utils::epin;
use husky_text::TextPosition;
use vm::*;
use word::WordOpr;

use super::*;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub(super) fn handle_word_opr(
        &mut self,
        word_opr: WordOpr,
        text_start: TextPosition,
    ) -> AtomResult<()> {
        match word_opr {
            WordOpr::And | WordOpr::Or => self.push(word_opr.into(), text_start),
            WordOpr::As => {
                let ty = get!(self, ranged_ty?);
                self.push(AtomVariant::Suffix(SuffixOpr::AsTy(ty)), text_start)
            }
        }
    }
}
