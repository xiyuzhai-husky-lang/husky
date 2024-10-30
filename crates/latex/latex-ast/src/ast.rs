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
use latex_math_opr::LxMathPunctuation;
use latex_prelude::{mode::LxMode, script::LxScriptKind};
use latex_token::{
    data::{
        math::{LxMathDelimiter, LxMathTokenData},
        rose::LxRoseTokenData,
    },
    storage::LxTokenStorage,
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
    token_storage: &'a mut LxTokenStorage,
    arena: &'a mut LxAstArena,
) -> LxAstIdxRange {
    let mut parser = LxAstParser::new(db, input, mode, token_storage, arena);
    parser.parse_asts()
}

impl<'a> LxAstParser<'a> {
    pub(crate) fn parse_asts(&mut self) -> LxAstIdxRange {
        match self.mode() {
            LxMode::Rose => todo!(),
            LxMode::Math => {
                let mut asts = vec![];
                while let Some(ast) = self.parse_math_ast() {
                    asts.push(ast)
                }
                self.alloc_math_asts(asts).into()
            }
        }
    }

    pub(crate) fn parse_math_asts(&mut self) -> LxMathAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_math_ast() {
            asts.push(ast)
        }
        self.alloc_math_asts(asts)
    }

    fn parse_math_ast(&mut self) -> Option<LxMathAstData> {
        let mut ast = self.parse_atomic_math_ast()?;
        match self.peek_math_token_data()? {
            // TODO include more cases, like \limits
            LxMathTokenData::Subscript | LxMathTokenData::Superscript => {
                let (idx, token) = self.next_math_token().unwrap();
                ast = match ast {
                    LxMathAstData::Attach { .. } => ast,
                    base => {
                        let base = self.alloc_math_ast(base);
                        LxMathAstData::Attach {
                            base,
                            scripts: Default::default(),
                        }
                        .into()
                    }
                    _ => todo!(),
                };
                let LxMathAstData::Attach {
                    ref mut scripts, ..
                } = ast
                else {
                    unreachable!()
                };
                let script_kind = match token {
                    LxMathTokenData::Subscript => LxScriptKind::Subscript,
                    LxMathTokenData::Superscript => LxScriptKind::Superscript,
                    _ => todo!(),
                };
                let ast = match self.parse_atomic_math_ast() {
                    Some(new_subscript) => self.alloc_math_ast(new_subscript),
                    None => todo!("err: expected subscript"),
                };
                scripts.push((script_kind, ast));
            }
            _ => (),
        };
        Some(ast)
    }
}

// TODO replace it with example
#[test]
fn parse_tex_input_into_asts_works() {
    use expect_test::Expect;

    fn t(input: &str, mode: LxMode, expected: Expect) {
        let db = &DB::default();
        let mut arena = LxAstArena::default();
        let mut token_storage = LxTokenStorage::default();
        let asts = parse_latex_input_into_asts(db, input, mode, &mut token_storage, &mut arena);
        expected.assert_debug_eq(&((token_storage, arena, asts).debug(db)));
    }
    t(
        "",
        LxMode::Math,
        expect![[r#"
            (
                LxTokenStorage {
                    ranged_math_tokens: [],
                    ranged_rose_tokens: [],
                },
                LxAstArena {
                    math: Arena {
                        data: [],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                LxAstIdxRange::Math(
                    ArenaIdxRange(
                        0..0,
                    ),
                ),
            )
        "#]],
    );
    t(
        "x",
        LxMode::Math,
        expect![[r#"
            (
                LxTokenStorage {
                    ranged_math_tokens: [
                        (
                            (
                                0,
                                1,
                            ),
                            [1:1, 1:2),
                            Letter(
                                LowerX,
                            ),
                        ),
                    ],
                    ranged_rose_tokens: [],
                },
                LxAstArena {
                    math: Arena {
                        data: [],
                    },
                    rose: Arena {
                        data: [],
                    },
                },
                LxAstIdxRange::Math(
                    ArenaIdxRange(
                        0..0,
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
                LxTokenStorage {
                    ranged_math_tokens: [
                        (
                            (
                                0,
                                1,
                            ),
                            [1:1, 1:2),
                            Letter(
                                LowerX,
                            ),
                        ),
                        (
                            (
                                1,
                                2,
                            ),
                            [1:2, 1:3),
                            Punctuation(
                                Add,
                            ),
                        ),
                        (
                            (
                                2,
                                3,
                            ),
                            [1:3, 1:4),
                            Digit(
                                One,
                            ),
                        ),
                    ],
                    ranged_rose_tokens: [],
                },
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxMathTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Opr(
                                LxMathTokenIdx(
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
                LxAstIdxRange::Math(
                    ArenaIdxRange(
                        0..2,
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
                LxTokenStorage {
                    ranged_math_tokens: [
                        (
                            (
                                0,
                                1,
                            ),
                            [1:1, 1:2),
                            Letter(
                                LowerX,
                            ),
                        ),
                        (
                            (
                                1,
                                2,
                            ),
                            [1:2, 1:3),
                            Superscript,
                        ),
                        (
                            (
                                2,
                                3,
                            ),
                            [1:3, 1:4),
                            Digit(
                                Two,
                            ),
                        ),
                    ],
                    ranged_rose_tokens: [],
                },
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxMathTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Digit(
                                LxMathTokenIdx(
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
                LxAstIdxRange::Math(
                    ArenaIdxRange(
                        2..3,
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
                LxTokenStorage {
                    ranged_math_tokens: [
                        (
                            (
                                0,
                                1,
                            ),
                            [1:1, 1:2),
                            Letter(
                                LowerX,
                            ),
                        ),
                        (
                            (
                                1,
                                2,
                            ),
                            [1:2, 1:3),
                            Subscript,
                        ),
                        (
                            (
                                2,
                                3,
                            ),
                            [1:3, 1:4),
                            Digit(
                                Two,
                            ),
                        ),
                    ],
                    ranged_rose_tokens: [],
                },
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxMathTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Digit(
                                LxMathTokenIdx(
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
                LxAstIdxRange::Math(
                    ArenaIdxRange(
                        2..3,
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
                LxTokenStorage {
                    ranged_math_tokens: [
                        (
                            (
                                0,
                                1,
                            ),
                            [1:1, 1:2),
                            Letter(
                                LowerX,
                            ),
                        ),
                        (
                            (
                                1,
                                2,
                            ),
                            [1:2, 1:3),
                            Superscript,
                        ),
                        (
                            (
                                2,
                                3,
                            ),
                            [1:3, 1:4),
                            LeftDelimiter(
                                Curl,
                            ),
                        ),
                        (
                            (
                                3,
                                4,
                            ),
                            [1:4, 1:5),
                            Letter(
                                LowerI,
                            ),
                        ),
                        (
                            (
                                4,
                                5,
                            ),
                            [1:5, 1:6),
                            Punctuation(
                                Add,
                            ),
                        ),
                        (
                            (
                                5,
                                6,
                            ),
                            [1:6, 1:7),
                            Digit(
                                Two,
                            ),
                        ),
                        (
                            (
                                6,
                                7,
                            ),
                            [1:7, 1:8),
                            RightDelimiter(
                                Curl,
                            ),
                        ),
                    ],
                    ranged_rose_tokens: [],
                },
                LxAstArena {
                    math: Arena {
                        data: [
                            LxMathAstData::Letter(
                                LxMathTokenIdx(
                                    0,
                                ),
                                LowerX,
                            ),
                            LxMathAstData::Letter(
                                LxMathTokenIdx(
                                    3,
                                ),
                                LowerI,
                            ),
                            LxMathAstData::Opr(
                                LxMathTokenIdx(
                                    4,
                                ),
                                Add,
                            ),
                            LxMathAstData::Digit(
                                LxMathTokenIdx(
                                    5,
                                ),
                                Two,
                            ),
                            LxMathAstData::Delimited {
                                left_delimiter_token_idx: LxMathTokenIdx(
                                    2,
                                ),
                                left_delimiter: Curl,
                                asts: ArenaIdxRange(
                                    1..4,
                                ),
                                right_delimiter_token_idx: LxMathTokenIdx(
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
                LxAstIdxRange::Math(
                    ArenaIdxRange(
                        5..6,
                    ),
                ),
            )
        "#]],
    );
}
