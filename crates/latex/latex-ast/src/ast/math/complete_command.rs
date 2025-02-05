use latex_token::token::spec::{LxSpecDelimiter, LxSpecTokenData};

use super::*;

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_math_complete_command(
        &mut self,
        command_token_idx: LxMathTokenIdx,
        command_signature: &LxCompleteCommandSignature,
    ) -> LxMathAstData {
        let command_path = command_signature.path();
        let mut arguments: SmallVec<[LxMathCompleteCommandArgument; 2]> = smallvec![];
        let options = self.parse_math_options();
        for parameter in command_signature.parameters() {
            arguments.push(self.parse_math_complete_command_argument(parameter.mode()));
        }
        LxMathAstData::CompleteCommand {
            command_token_idx,
            command_path,
            arguments,
        }
    }

    fn parse_math_options(&mut self) -> Option<()> {
        match self.peek_math_token_data()? {
            LxMathTokenData::Punctuation(LxMathPunctuation::Lbox) => (),
            _ => return None,
        }
        let Some((lbox_token_idx, LxMathTokenData::Punctuation(LxMathPunctuation::Lbox))) =
            self.next_math_token()
        else {
            unreachable!("we just peeked a left box")
        };
        // TODO: ad hoc
        while let Some((_, token)) = self.next_spec_token() {
            match token {
                LxSpecTokenData::RightDelimiter(LxSpecDelimiter::Box) => break,
                _ => (),
            }
        }
        Some(())
    }

    fn parse_math_complete_command_argument(
        &mut self,
        mode: LxCommandParameterMode,
    ) -> LxMathCompleteCommandArgument {
        match self.peek_math_token_data() {
            Some(math_token_data) => match math_token_data {
                LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl) => (),
                _ => {
                    if mode == LxCommandParameterMode::Math {
                        let Some(math_ast) = self.parse_atomic_math_ast() else {
                            todo!()
                        };
                        let math_ast = self.alloc_math_ast(math_ast);
                        return LxMathCompleteCommandArgument::MathAst(math_ast);
                    } else {
                        todo!()
                    }
                }
            },
            None => todo!(),
        }
        let Some((lcurl_token_idx, LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl))) =
            self.next_math_token()
        else {
            unreachable!()
        };
        let asts = match mode {
            LxCommandParameterMode::Math => LxMathCommandArgumentAsts::Math(self.parse_math_asts()),
            LxCommandParameterMode::Rose => LxMathCommandArgumentAsts::Rose(self.parse_rose_asts()),
            LxCommandParameterMode::Name => todo!(),
            LxCommandParameterMode::SingleLetter => todo!(),
        };
        let Some((rcurl_token_idx, LxMathTokenData::RightDelimiter(LxMathDelimiter::Curl))) =
            self.next_math_token()
        else {
            todo!("report error properly")
        };
        LxMathCompleteCommandArgument::Asts {
            lcurl_token_idx,
            asts,
            rcurl_token_idx,
        }
    }
}
