use crate::*;
use husky_ast::AstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::*;
use husky_manifest::ManifestJar;
use husky_symbol::{LocalSymbolSheet, SymbolContext};
use husky_token::TokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

pub(crate) fn quick_parse(
    db: &DB,
    input: &str,
    local_symbol_sheet: &mut LocalSymbolSheet,
) -> (ExprSheet, Option<ExprIdx>) {
    let mut expr_sheet = ExprSheet::default();
    let toolchain = db.dev_toolchain().unwrap();
    let path_menu = db.path_menu(toolchain).unwrap();
    let crate_prelude = db.crate_prelude(path_menu.core_library()).unwrap();
    let ctx = SymbolContext::new(db, crate_prelude, local_symbol_sheet);
    let tokens = db.tokenize(input);
    let token_sheet = TokenSheet::new(tokens);
    let token_iter = token_sheet
        .token_group_token_stream(token_sheet.token_group_iter().next().unwrap().0, None);
    let expr_idx = parse_expr(ctx, token_iter, &mut expr_sheet);
    (expr_sheet, expr_idx)
}

#[test]
fn parse_expr_works() {
    let mut local_symbol_sheet = LocalSymbolSheet::default();
    let db = DB::default();
    macro_rules! t {
        ($input: expr) => {{
            quick_parse(&db, $input, &mut local_symbol_sheet)
        }};
    }

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                        ),
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("1"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                1,
                            ),
                        ),
                        Opn {
                            opn: Prefix(
                                Minus,
                            ),
                            opds: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("-1"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                        ),
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("1i32"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                        ),
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("2i64"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Err(
                            Token(
                                ParseIntError,
                            ),
                        ),
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("22222222222222222222222222222222222222222222i64"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                        ),
                        Literal(
                            TokenIdx(
                                2,
                            ),
                        ),
                        Opn {
                            opn: Binary(
                                PureClosed(
                                    Add,
                                ),
                            ),
                            opds: ArenaIdxRange(
                                0..2,
                            ),
                        },
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("1 + 1"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Opn {
                            opn: List {
                                opr: NewVec,
                                bracket: Box,
                                bra_token_idx: TokenIdx(
                                    0,
                                ),
                                ket_token_idx: TokenIdx(
                                    1,
                                ),
                            },
                            opds: ArenaIdxRange(
                                0..0,
                            ),
                        },
                        Unrecognized(
                            Identifier(
                                Word(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        ),
                        Opn {
                            opn: Application,
                            opds: ArenaIdxRange(
                                0..2,
                            ),
                        },
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("[]i32"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                1,
                            ),
                        ),
                        Opn {
                            opn: List {
                                opr: NewVec,
                                bracket: Box,
                                bra_token_idx: TokenIdx(
                                    0,
                                ),
                                ket_token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            opds: ArenaIdxRange(
                                0..1,
                            ),
                        },
                        Unrecognized(
                            Identifier(
                                Word(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        ),
                        Opn {
                            opn: Application,
                            opds: ArenaIdxRange(
                                1..3,
                            ),
                        },
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                3,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("[3]i32"));

    expect_test::expect![[r#"
        (
            ExprSheet {
                expr_arena: Arena {
                    data: [
                        Opn {
                            opn: List {
                                opr: NewVec,
                                bracket: Box,
                                bra_token_idx: TokenIdx(
                                    0,
                                ),
                                ket_token_idx: TokenIdx(
                                    1,
                                ),
                            },
                            opds: ArenaIdxRange(
                                0..0,
                            ),
                        },
                        Unrecognized(
                            Identifier(
                                Word(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        ),
                        Opn {
                            opn: Application,
                            opds: ArenaIdxRange(
                                0..2,
                            ),
                        },
                    ],
                },
                entity_path_expr_arena: Arena {
                    data: [],
                },
                pattern_expr_arena: Arena {
                    data: [],
                },
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t!("(i32, i32)"));
}
