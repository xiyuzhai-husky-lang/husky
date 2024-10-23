mod code;
pub mod math;
pub mod rose;
pub mod tree;

use self::{code::TexCodeAstData, math::LxMathAstData, rose::TexRoseAstData, tree::TexTreeAstData};
use crate::parser::TexAstParser;
#[cfg(test)]
use crate::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use latex_math_letter::LxMathLetter;
use latex_math_opr::LxMathOpr;
use latex_prelude::mode::TexMode;
use latex_token::data::{math::LxMathTokenData, rose::TexRoseTokenData, TexTokenData};

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexAstData {
    Code(TexCodeAstData),
    Math(LxMathAstData),
    Rose(TexRoseAstData),
    Tree(TexTreeAstData),
}

pub type TexAstArena = Arena<TexAstData>;
pub type TexAstIdx = ArenaIdx<TexAstData>;
pub type TexAstIdxRange = ArenaIdxRange<TexAstData>;

pub fn parse_tex_input_into_asts<'a>(
    db: &'a ::salsa::Db,
    input: &'a str,
    mode: TexMode,
    arena: &'a mut TexAstArena,
) -> TexAstIdxRange {
    let mut parser = TexAstParser::new(db, input, mode, arena);
    let asts = parser.parse_asts();
    asts
}

impl<'a> TexAstParser<'a> {
    pub(crate) fn parse_asts(&mut self) -> TexAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_ast() {
            asts.push(ast)
        }
        self.alloc_asts(asts)
    }

    fn parse_ast(&mut self) -> Option<TexAstData> {
        let mut ast = self.parse_atomic_ast()?;
        match self.peek_token()? {
            TexTokenData::Math(LxMathTokenData::Subscript | LxMathTokenData::Superscript) => {
                let (idx, TexTokenData::Math(token)) = self.next_token().unwrap() else {
                    unreachable!()
                };
                ast = match ast {
                    TexAstData::Math(LxMathAstData::Attach {
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
                let TexAstData::Math(LxMathAstData::Attach {
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

    fn parse_atomic_ast(&mut self) -> Option<TexAstData> {
        match self.peek_token()? {
            TexTokenData::Math(token) => {
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
            TexTokenData::Rose(token) => match token {
                TexRoseTokenData::Word(_) => todo!(),
                TexRoseTokenData::Command(_) => todo!(),
                TexRoseTokenData::Dollar => todo!(),
                TexRoseTokenData::Nat32(_) => todo!(),
                TexRoseTokenData::NewParagraph => todo!(),
            },
            TexTokenData::Code(_) => todo!(),
        }
        let (idx, token) = self.next_token().unwrap();
        Some(match token {
            TexTokenData::Math(token) => self.parse_atomic_math_ast(idx, token).into(),
            TexTokenData::Rose(token) => self.parse_atomic_text_ast(idx, token).into(),
            TexTokenData::Code(token) => todo!(),
        })
    }
}

#[test]
fn parse_tex_input_into_asts_works() {
    use expect_test::Expect;

    fn t(input: &str, mode: TexMode, expected: Expect) {
        let db = &DB::default();
        let mut arena = TexAstArena::default();
        let asts = parse_tex_input_into_asts(db, input, mode, &mut arena);
        expected.assert_debug_eq(&((arena, asts).debug(db)));
    }
    t(
        "",
        TexMode::Math,
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
        TexMode::Math,
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
        TexMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        TexAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        TexAstData::Math(
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
        TexMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        TexAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        TexAstData::Math(
                            LxMathAstData::Nat32(
                                2,
                            ),
                        ),
                        TexAstData::Math(
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
        TexMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        TexAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        TexAstData::Math(
                            LxMathAstData::Nat32(
                                2,
                            ),
                        ),
                        TexAstData::Math(
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
        TexMode::Math,
        expect![[r#"
            (
                Arena {
                    data: [
                        TexAstData::Math(
                            LxMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        TexAstData::Math(
                            LxMathAstData::Letter(
                                LowerI,
                            ),
                        ),
                        TexAstData::Math(
                            LxMathAstData::Opr(
                                Add,
                            ),
                        ),
                        TexAstData::Math(
                            LxMathAstData::Nat32(
                                2,
                            ),
                        ),
                        TexAstData::Math(
                            LxMathAstData::Delimited {
                                left_delimiter_token_idx: TexTokenIdx(
                                    2,
                                ),
                                left_delimiter: Curl,
                                asts: ArenaIdxRange(
                                    1..4,
                                ),
                                right_delimiter_token_idx: TexTokenIdx(
                                    6,
                                ),
                                right_delimiter: Curl,
                            },
                        ),
                        TexAstData::Math(
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
