use husky_opn_syntax::RawSuffixOpr;
use husky_text::TextPosition;
use husky_word::WordOpr;

use super::*;

impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
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
                let ty = deprecated_get!(self, ranged_ty?);
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    HuskyAtomVariant::Suffix(RawSuffixOpr::AsTy(ty)),
                ))
            }
            WordOpr::Be => {
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    HuskyAtomVariant::Be,
                ))?;
                let (pattern, other_atoms) = self.parse_pattern()?;
                self.stack.pop_unwrap_ignore();
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    HuskyAtomVariant::BePattern(pattern),
                ))?;
                for atom in other_atoms {
                    self.stack.push(atom)?
                }
                Ok(())
            }
        }
    }
}
