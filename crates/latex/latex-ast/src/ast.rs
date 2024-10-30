pub mod math;
pub mod rose;

use self::{
    math::{LxMathAstArena, LxMathAstArenaMap, LxMathAstArenaRef, LxMathAstData},
    rose::{LxRoseAstArena, LxRoseAstArenaMap, LxRoseAstArenaRef, LxRoseAstData},
};
use crate::parser::LxAstParser;
#[cfg(test)]
use crate::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_math_letter::LxMathLetter;
use latex_math_opr::LxMathOpr;
use latex_prelude::{mode::LxMode, script::LxScriptKind};
use latex_token::{
    data::{
        math::{LxMathDelimiter, LxMathTokenData},
        rose::LxRoseTokenData,
        LxTokenData,
    },
    lexer::lex_latex_input,
};
use math::{LxMathAstIdx, LxMathAstIdxRange};
use rose::{LxRoseAstIdx, LxRoseAstIdxRange};

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxAstData {
    Math(LxMathAstData),
    Rose(LxRoseAstData),
}

#[salsa::derive_debug_with_db]
#[derive(Default, Debug)]
pub struct LxAstArena {
    pub(crate) math: LxMathAstArena,
    pub(crate) rose: LxRoseAstArena,
}
impl LxAstArena {
    pub(crate) fn as_arena_ref(&self) -> LxAstArenaRef {
        LxAstArenaRef {
            math: self.math.as_arena_ref(),
            rose: self.rose.as_arena_ref(),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct LxAstArenaRef<'a> {
    math: LxMathAstArenaRef<'a>,
    rose: LxRoseAstArenaRef<'a>,
}

impl<'a> LxAstArenaRef<'a> {
    pub fn math(&self) -> LxMathAstArenaRef<'a> {
        self.math
    }

    pub fn rose(&self) -> LxRoseAstArenaRef<'a> {
        self.rose
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct LxAstArenaMap<T> {
    pub(crate) math: LxMathAstArenaMap<T>,
    pub(crate) rose: LxRoseAstArenaMap<T>,
}

impl<T> LxAstArenaMap<T> {
    pub(crate) fn new(arena: &LxAstArena) -> Self {
        Self {
            math: LxMathAstArenaMap::new(&arena.math),
            rose: LxRoseAstArenaMap::new(&arena.rose),
        }
    }
}

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxAstIdx {
    Math(LxMathAstIdx),
    Rose(LxRoseAstIdx),
}

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxAstIdxRange {
    Math(LxMathAstIdxRange),
    Rose(LxRoseAstIdxRange),
}

pub fn parse_latex_input_into_asts<'a>(
    db: &'a ::salsa::Db,
    input: &'a str,
    mode: LxMode,
    arena: &'a mut LxAstArena,
) -> Option<LxAstIdxRange> {
    let tokens = lex_latex_input(input, mode, db);
    let mut parser = LxAstParser::new(db, &tokens, mode, arena);
    let asts = parser.parse_asts();
    asts
}

impl<'a> LxAstParser<'a> {
    pub(crate) fn parse_asts(&mut self) -> Option<LxAstIdxRange> {
        match self.peek_token()? {
            LxTokenData::Math(_) => {
                let mut asts = vec![];
                while let Some(ast) = self.parse_ast() {
                    let LxAstData::Math(ast) = ast else { todo!() };
                    asts.push(ast)
                }
                Some(self.alloc_math_asts(asts).into())
            }
            LxTokenData::Rose(_) => todo!(),
        }
    }

    pub(crate) fn parse_math_asts(&mut self) -> LxMathAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_ast() {
            let LxAstData::Math(ast) = ast else { todo!() };
            asts.push(ast)
        }
        self.alloc_math_asts(asts)
    }

    fn parse_ast(&mut self) -> Option<LxAstData> {
        let mut ast = self.parse_atomic_ast()?;
        match self.peek_token()? {
            // TODO include more cases, like \limits
            LxTokenData::Math(LxMathTokenData::Subscript | LxMathTokenData::Superscript) => {
                let (idx, LxTokenData::Math(token)) = self.next_token().unwrap() else {
                    unreachable!()
                };
                ast = match ast {
                    LxAstData::Math(LxMathAstData::Attach { .. }) => ast,
                    LxAstData::Math(base) => {
                        let base = self.alloc_math_ast(base);
                        LxMathAstData::Attach {
                            base,
                            scripts: Default::default(),
                        }
                        .into()
                    }
                    _ => todo!(),
                };
                let LxAstData::Math(LxMathAstData::Attach {
                    ref mut scripts, ..
                }) = ast
                else {
                    unreachable!()
                };
                let script_kind = match token {
                    LxMathTokenData::Subscript => LxScriptKind::Subscript,
                    LxMathTokenData::Superscript => LxScriptKind::Superscript,
                    _ => todo!(),
                };
                let ast = match self.parse_atomic_ast() {
                    Some(LxAstData::Math(new_subscript)) => self.alloc_math_ast(new_subscript),
                    Some(LxAstData::Rose(_)) => todo!("err: expected math ast"),
                    None => todo!("err: expected subscript"),
                };
                scripts.push((script_kind, ast));
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
                    LxMathTokenData::Digit(_) => (),
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
        let asts = parse_latex_input_into_asts(db, input, mode, &mut arena);
        expected.assert_debug_eq(&((arena, asts).debug(db)));
    }
    t(
        "",
        LxMode::Math,
        expect![[r#"
            (
                LxAstArena {
                    math: Arena {
                        data: [],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                None,
            )
        "#]],
    );
    t(
        "x",
        LxMode::Math,
        expect![[r#"
            (
                LxAstArena {
                    math: Arena {
                        data: [],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                Some(
                    LxAstIdxRange::Math(
                        ArenaIdxRange(
                            0..0,
                        ),
                    ),
                ),
            )
        "#]],
    );
    t(
        "x+1",
        LxMode::Math,
        expect![[r#"
            (
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Opr(
                                LxTokenIdx(
                                    1,
                                ),
                                Add,
                            ),
                        ],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                Some(
                    LxAstIdxRange::Math(
                        ArenaIdxRange(
                            0..2,
                        ),
                    ),
                ),
            )
        "#]],
    );
    t(
        "x^2",
        LxMode::Math,
        expect![[r#"
            (
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Digit(
                                LxTokenIdx(
                                    2,
                                ),
                                Two,
                            ),
                            LxMathAstData::Attach {
                                base: 0,
                                scripts: [
                                    (
                                        LxScriptKind::Superscript,
                                        1,
                                    ),
                                ],
                            },
                        ],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                Some(
                    LxAstIdxRange::Math(
                        ArenaIdxRange(
                            2..3,
                        ),
                    ),
                ),
            )
        "#]],
    );
    t(
        "x_2",
        LxMode::Math,
        expect![[r#"
            (
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Digit(
                                LxTokenIdx(
                                    2,
                                ),
                                Two,
                            ),
                            LxMathAstData::Attach {
                                base: 0,
                                scripts: [
                                    (
                                        LxScriptKind::Subscript,
                                        1,
                                    ),
                                ],
                            },
                        ],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                Some(
                    LxAstIdxRange::Math(
                        ArenaIdxRange(
                            2..3,
                        ),
                    ),
                ),
            )
        "#]],
    );
    t(
        "x^{i+2}",
        LxMode::Math,
        expect![[r#"
            (
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Letter(
                                LxTokenIdx(
                                    3,
                                ),
                                LowerI,
                            ),
                            LxMathAstData::Opr(
                                LxTokenIdx(
                                    4,
                                ),
                                Add,
                            ),
                            LxMathAstData::Digit(
                                LxTokenIdx(
                                    5,
                                ),
                                Two,
                            ),
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
                            LxMathAstData::Attach {
                                base: 0,
                                scripts: [
                                    (
                                        LxScriptKind::Superscript,
                                        4,
                                    ),
                                ],
                            },
                        ],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                Some(
                    LxAstIdxRange::Math(
                        ArenaIdxRange(
                            5..6,
                        ),
                    ),
                ),
            )
        "#]],
    );
}
