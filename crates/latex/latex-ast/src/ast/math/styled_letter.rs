use super::*;

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_styled_letter(
        &mut self,
        style_command_token_idx: LxMathTokenIdx,
        style: LxMathLetterStyle,
    ) -> LxMathAstData {
        let Some((style_lcurl_token_idx, style_lcurl_token)) = self.next_math_token() else {
            todo!()
        };
        match style_lcurl_token {
            LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl) => (),
            _ => todo!(),
        };
        let Some((plain_letter_token_idx, plain_letter_token)) = self.next_math_token() else {
            todo!()
        };
        let LxMathTokenData::Letter(plain_letter) = plain_letter_token else {
            todo!()
        };
        let Some((style_rcurl_token_idx, style_rcurl_token)) = self.next_math_token() else {
            todo!()
        };
        match style_rcurl_token {
            LxMathTokenData::RightDelimiter(LxMathDelimiter::Curl) => (),
            _ => todo!(),
        };
        let styled_letter = match style.apply(plain_letter) {
            Ok(styled_letter) => styled_letter,
            Err(e) => todo!("{}", e),
        };
        LxMathAstData::StyledLetter {
            style_command_token_idx,
            style_lcurl_token_idx,
            plain_letter_token_idx,
            style_rcurl_token_idx,
            style,
            styled_letter,
        }
    }
}
