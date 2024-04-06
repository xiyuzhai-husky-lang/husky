pub mod math;
pub mod text;

use self::{math::TexMathAstData, text::TexTextAstData};
use crate::{parser::TexAstParser, sheet::TexAstSheet, *};
use husky_tex_math_letter::TexMathLetter;
use husky_tex_math_opr::TexMathOpr;
use husky_tex_prelude::mode::TexMode;
use husky_tex_token::data::{math::TexMathTokenData, text::TexTextTokenData, TexTokenData};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TexAstData {
    Math(TexMathAstData),
    Text(TexTextAstData),
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
                1..1,
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
                    1..1,
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
                            TexMathAstData::Letter(
                                LowerX,
                            ),
                        ),
                        TexAstData::Math(
                            TexMathAstData::Opr(
                                Add,
                            ),
                        ),
                    ],
                },
                ArenaIdxRange(
                    1..3,
                ),
            )
        "#]],
    );
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
        match self.peek()? {
            TexTokenData::Math(token) => match token {
                TexMathTokenData::Subscript => todo!(),
                TexMathTokenData::Superscript => todo!(),
                _ => (),
            },
            TexTokenData::Text(token) => (),
        };
        Some(ast)
    }

    fn parse_atomic_ast(&mut self) -> Option<TexAstData> {
        match self.peek()? {
            TexTokenData::Math(token) => {
                match token {
                    TexMathTokenData::Command(_) => todo!(),
                    TexMathTokenData::LeftDelimiter(_) => todo!(),
                    TexMathTokenData::RightDelimiter(_) => todo!(),
                    TexMathTokenData::Letter(_) => (),
                    TexMathTokenData::Opr(_) => (),
                    TexMathTokenData::Nat32(_) => (),
                    TexMathTokenData::Other(_) => todo!(),
                    TexMathTokenData::Subscript => todo!(),
                    TexMathTokenData::Superscript => todo!(),
                };
            }
            TexTokenData::Text(token) => match token {
                TexTextTokenData::Word(_) => todo!(),
                TexTextTokenData::Command(_) => todo!(),
                TexTextTokenData::Dollar => todo!(),
                TexTextTokenData::Nat32(_) => todo!(),
            },
        }
        let (idx, token) = self.next_token().unwrap();
        Some(match token {
            TexTokenData::Math(token) => self.parse_atomic_math_ast(token).into(),
            TexTokenData::Text(token) => self.parse_atomic_text_ast(token).into(),
        })
    }
}
