use super::*;

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_lefted(&mut self, left_command_token_idx: LxMathTokenIdx) -> LxMathAstData {
        let Some(data) = self.parse_math_ast() else {
            todo!()
        };
        let argument = self.alloc_math_ast(data);
        LxMathAstData::Lefted {
            left_command_token_idx,
            argument,
        }
    }

    pub(super) fn parse_righted(
        &mut self,
        right_command_token_idx: LxMathTokenIdx,
    ) -> LxMathAstData {
        let Some(data) = self.parse_math_ast() else {
            todo!()
        };
        match data {
            LxMathAstData::Punctuation(..) => (),
            _ => todo!("this could be caused by sloppy latex code like $\\left(a+b\\right)^2$"),
        }
        let argument = self.alloc_math_ast(data);
        LxMathAstData::Righted {
            right_command_token_idx,
            argument,
        }
    }
}
