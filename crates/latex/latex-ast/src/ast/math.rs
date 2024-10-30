use latex_token::{
    data::math::{digit::LxMathDigit, LxMathDelimiter},
    idx::LxTokenIdx,
};

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxMathAstData {
    Letter(LxMathLetter),
    Opr(LxMathOpr),
    Digit(LxMathDigit),
    /// not obtained through parsing, but through ui
    TextEdit {
        buffer: String,
    },
    Attach {
        base: LxAstIdx,
        scripts: Vec<(LxScriptKind, LxAstIdx)>,
    },
    Delimited {
        left_delimiter_token_idx: LxTokenIdx,
        left_delimiter: LxMathDelimiter,
        asts: LxAstIdxRange,
        right_delimiter_token_idx: LxTokenIdx,
        right_delimiter: LxMathDelimiter,
    },
}

impl<'a> LxAstParser<'a> {
    pub(super) fn parse_atomic_math_ast(
        &mut self,
        idx: LxTokenIdx,
        token: LxMathTokenData,
    ) -> LxMathAstData {
        match token {
            LxMathTokenData::Command(_) => todo!(),
            LxMathTokenData::LeftDelimiter(delimiter) => self.parse_delimited(idx, delimiter),
            LxMathTokenData::RightDelimiter(_) => todo!(),
            LxMathTokenData::Letter(letter) => LxMathAstData::Letter(letter),
            LxMathTokenData::Opr(opr) => LxMathAstData::Opr(opr), // it's not constructed into a tree yet in the ast stage
            LxMathTokenData::Digit(digit) => LxMathAstData::Digit(digit),
            LxMathTokenData::Other(_) => todo!(),
            LxMathTokenData::Subscript => todo!(),
            LxMathTokenData::Superscript => todo!(),
            LxMathTokenData::Error(_) => todo!(),
        }
    }

    // here we differ from the latex syntax, we see all possible delimiters as latex delimiters
    fn parse_delimited(
        &mut self,
        left_delimiter_token_idx: LxTokenIdx,
        left_delimiter: LxMathDelimiter,
    ) -> LxMathAstData {
        let asts = self.parse_asts();
        let Some((idx, token, _, _)) = self.next_token() else {
            todo!()
        };
        match token {
            LxTokenData::Math(token) => match token {
                LxMathTokenData::Command(_) => todo!(),
                LxMathTokenData::LeftDelimiter(_) => todo!(),
                LxMathTokenData::RightDelimiter(right_delimiter) => LxMathAstData::Delimited {
                    left_delimiter_token_idx,
                    left_delimiter,
                    asts,
                    right_delimiter_token_idx: idx,
                    right_delimiter,
                },
                LxMathTokenData::Letter(_) => todo!(),
                LxMathTokenData::Opr(_) => todo!(),
                LxMathTokenData::Digit(_) => todo!(),
                LxMathTokenData::Other(_) => todo!(),
                LxMathTokenData::Subscript => todo!(),
                LxMathTokenData::Superscript => todo!(),
                LxMathTokenData::Error(_) => todo!(),
            },
            LxTokenData::Rose(_) => todo!(),
        }
    }
}
