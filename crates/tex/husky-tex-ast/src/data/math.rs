use husky_tex_token::{data::math::TexMathDelimiter, idx::TexTokenIdx};

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexMathAstData {
    Letter(TexMathLetter),
    Opr(TexMathOpr),
    Nat32(u32),
    TextEdit {
        buffer: String,
    },
    Attach {
        base: TexAstIdx,
        superscript: Option<TexAstIdx>,
        subscript: Option<TexAstIdx>,
    },
    Delimited {
        left_delimiter_token_idx: TexTokenIdx,
        left_delimiter: TexMathDelimiter,
        asts: TexAstIdxRange,
        right_delimiter_token_idx: TexTokenIdx,
        right_delimiter: TexMathDelimiter,
    },
}

impl<'a> TexAstParser<'a> {
    pub(super) fn parse_atomic_math_ast(
        &mut self,
        idx: TexTokenIdx,
        token: TexMathTokenData,
    ) -> TexMathAstData {
        match token {
            TexMathTokenData::Command(_) => todo!(),
            TexMathTokenData::LeftDelimiter(delimiter) => self.parse_delimited(idx, delimiter),
            TexMathTokenData::RightDelimiter(_) => todo!(),
            TexMathTokenData::Letter(letter) => TexMathAstData::Letter(letter),
            TexMathTokenData::Opr(opr) => TexMathAstData::Opr(opr),
            TexMathTokenData::Nat32(number) => TexMathAstData::Nat32(number),
            TexMathTokenData::Other(_) => todo!(),
            TexMathTokenData::Subscript => todo!(),
            TexMathTokenData::Superscript => todo!(),
        }
    }

    fn parse_delimited(
        &mut self,
        left_delimiter_token_idx: TexTokenIdx,
        left_delimiter: TexMathDelimiter,
    ) -> TexMathAstData {
        let asts = self.parse_asts();
        let Some((idx, token)) = self.next_token() else {
            todo!()
        };
        match token {
            TexTokenData::Math(token) => match token {
                TexMathTokenData::Command(_) => todo!(),
                TexMathTokenData::LeftDelimiter(_) => todo!(),
                TexMathTokenData::RightDelimiter(right_delimiter) => TexMathAstData::Delimited {
                    left_delimiter_token_idx,
                    left_delimiter,
                    asts,
                    right_delimiter_token_idx: idx,
                    right_delimiter,
                },
                TexMathTokenData::Letter(_) => todo!(),
                TexMathTokenData::Opr(_) => todo!(),
                TexMathTokenData::Nat32(_) => todo!(),
                TexMathTokenData::Other(_) => todo!(),
                TexMathTokenData::Subscript => todo!(),
                TexMathTokenData::Superscript => todo!(),
            },
            TexTokenData::Rose(_) => todo!(),
            TexTokenData::Code(_) => todo!(),
        }
    }
}
