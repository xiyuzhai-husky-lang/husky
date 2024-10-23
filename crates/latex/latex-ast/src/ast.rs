mod code;
pub mod math;
pub mod rose;
pub mod tree;

use self::{code::LxCodeAstData, math::LxMathAstData, rose::LxRoseAstData, tree::LxTreeAstData};
use crate::parser::LxAstParser;
#[cfg(test)]
use crate::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use latex_math_letter::LxMathLetter;
use latex_math_opr::LxMathOpr;
use latex_prelude::mode::LxMode;
use latex_token::data::{math::LxMathTokenData, rose::LxRoseTokenData, LxTokenData};

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxAstData {
    Code(LxCodeAstData),
    Math(LxMathAstData),
    Rose(LxRoseAstData),
    Tree(LxTreeAstData),
}

pub type LxAstArena = Arena<LxAstData>;
pub type LxAstIdx = ArenaIdx<LxAstData>;
pub type LxAstIdxRange = ArenaIdxRange<LxAstData>;

pub fn parse_tex_input_into_asts<'a>(
    db: &'a ::salsa::Db,
    input: &'a str,
    mode: LxMode,
    arena: &'a mut LxAstArena,
) -> LxAstIdxRange {
    let mut parser = LxAstParser::new(db, input, mode, arena);
    let asts = parser.parse_asts();
    asts
}

impl<'a> LxAstParser<'a> {
    pub(crate) fn parse_asts(&mut self) -> LxAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_ast() {
            asts.push(ast)
        }
        self.alloc_asts(asts)
    }

    fn parse_ast(&mut self) -> Option<LxAstData> {
        let mut ast = self.parse_atomic_ast()?;
        match self.peek_token()? {
            LxTokenData::Math(LxMathTokenData::Subscript | LxMathTokenData::Superscript) => {
                let (idx, LxTokenData::Math(token)) = self.next_token().unwrap() else {
                    unreachable!()
                };
                ast = match ast {
                    LxAstData::Math(LxMathAstData::Attach {
                        base,
                        superscript,
                        subscript,
                    }) => ast,
                    base => {
                        let base = self.alloc_ast(base.into());
                        LxMathAstData::Attach {
                            base,
                            superscript: None,
                            subscript: None,
                        }
                        .into()
                    }
                };
                let LxAstData::Math(LxMathAstData::Attach {
                    superscript,
                    subscript,
                    ..
                }) = &mut ast
                else {
                    unreachable!()
                };
                match token {
                    LxMathTokenData::Subscript => match self.parse_atomic_ast() {
                        Some(new_subscript) => match subscript {
                            Some(_) => todo!("err: expected subscript"),
                            None => *subscript = Some(self.alloc_ast(new_subscript)),
                        },
                        None => todo!("err: expected subscript"),
                    },
                    LxMathTokenData::Superscript => match self.parse_atomic_ast() {
                        Some(new_superscript) => match superscript {
                            Some(_) => todo!(),
                            None => *superscript = Some(self.alloc_ast(new_superscript)),
                        },
                        None => todo!("err: expected superscript"),
                    },
                    _ => unreachable!(),
                }
            }
            _ => (),
        };
        Some(ast)
    }

    fn parse_atomic_ast(&mut self) -> Option<LxAstData> {
        match self.peek_token()? {
            LxTokenData::Math(token) => {
                match token {
                    LxMathTokenData::Command(_) => todo!(),
                    LxMathTokenData::LeftDelimiter(_) => (),
                    LxMathTokenData::RightDelimiter(_) => return None,
                    LxMathTokenData::Letter(_) => (),
                    LxMathTokenData::Opr(_) => (),
                    LxMathTokenData::Nat32(_) => (),
                    LxMathTokenData::Other(_) => todo!(),
                    LxMathTokenData::Subscript => todo!(),
                    LxMathTokenData::Superscript => todo!(),
                    LxMathTokenData::Error(_) => todo!(),
                };
            }
            LxTokenData::Rose(token) => match token {
                LxRoseTokenData::Word(_) => todo!(),
                LxRoseTokenData::Command(_) => todo!(),
                LxRoseTokenData::Dollar => todo!(),
                LxRoseTokenData::EscapedLpar => todo!(),
                LxRoseTokenData::EscapedLbox => todo!(),
                LxRoseTokenData::Nat32(_) => todo!(),
                LxRoseTokenData::NewParagraph => todo!(),
            },
        }
        let (idx, token) = self.next_token().unwrap();
        Some(match token {
            LxTokenData::Math(token) => self.parse_atomic_math_ast(idx, token).into(),
            LxTokenData::Rose(token) => self.parse_atomic_text_ast(idx, token).into(),
        })
    }
}

#[test]
fn parse_tex_input_into_asts_works() {
    use expect_test::Expect;

    fn t(input: &str, mode: LxMode, expected: Expect) {
        let db = &DB::default();
        let mut arena = LxAstArena::default();
        let asts = parse_tex_input_into_asts(db, input, mode, &mut arena);
        expected.assert_debug_eq(&((arena, asts).debug(db)));
    }
    t(
        "",
        LxMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [],
                },
                ArenaIdxRange(
                    0..0,
                ),
            )
        "#]],
    );
    t(
        "x",
        LxMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [],
                },
                ArenaIdxRange(
                    0..0,
                ),
            )
        "#]],
    );
    t(
        "x+1",
        LxMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        LxAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Opr(
                                Add,
                            ),
                        ),
                    ],
                },
                ArenaIdxRange(
                    0..2,
                ),
            )
        "#]],
    );
    t(
        "x^2",
        LxMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        LxAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Nat32(
                                2,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Attach {
                                base: 0,
                                superscript: Some(
                                    1,
                                ),
                                subscript: None,
                            },
                        ),
                    ],
                },
                ArenaIdxRange(
                    2..3,
                ),
            )
        "#]],
    );
    t(
        "x_2",
        LxMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        LxAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Nat32(
                                2,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Attach {
                                base: 0,
                                superscript: None,
                                subscript: Some(
                                    1,
                                ),
                            },
                        ),
                    ],
                },
                ArenaIdxRange(
                    2..3,
                ),
            )
        "#]],
    );
    t(
        "x^{i+2}",
        LxMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        LxAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Letter(
                                LowerI,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Opr(
                                Add,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Nat32(
                                2,
                            ),
                        ),
                        LxAstData::Math(
                            LxMathAstData::Delimited {
                                left_delimiter_token_idx: LxTokenIdx(
                                    2,
                                ),
                                left_delimiter: Curl,
                                asts: ArenaIdxRange(
                                    1..4,
                                ),
                                right_delimiter_token_idx: LxTokenIdx(
                                    6,
                                ),
                                right_delimiter: Curl,
                            },
                        ),
                        LxAstData::Math(
                            LxMathAstData::Attach {
                                base: 0,
                                superscript: Some(
                                    4,
                                ),
                                subscript: None,
                            },
                        ),
                    ],
                },
                ArenaIdxRange(
                    5..6,
                ),
            )
        "#]],
    );
}
