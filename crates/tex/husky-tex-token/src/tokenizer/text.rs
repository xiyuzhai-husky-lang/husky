use super::*;

impl<'a> TexTokenizer<'a> {
    pub(super) fn next_text_token_data(&mut self) -> Option<TexTextTokenData> {
        use husky_print_utils::p;

        let offset = self.chars.current_offset();
        match self.chars.next()? {
            numeric if numeric.is_numeric() => todo!(),
            '{' => todo!(),
            other => {
                p!(self
                    .chars
                    .get_str_slice_from_start_with(offset, |c| c.is_alphabetic()));
                todo!()
            }
        }
    }
}
