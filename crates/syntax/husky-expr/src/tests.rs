pub(crate) use husky_vfs::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::*;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    ExprJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

pub(crate) fn t<'a>(db: &'a DB, input: &str) -> &'a (ExprRegion, Option<ExprIdx>) {
    let toolchain = db.dev_toolchain().unwrap();
    let path_menu = db.vfs_path_menu(toolchain);
    let snippet = Snippet::new(db, input.to_owned());
    parse_expr_from_snippet(db, path_menu.core_library(), snippet)
        .as_ref()
        .unwrap()
}

#[test]
fn parse_expr_works() {
    let db = DB::default();
    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 1,
                },
            ),
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 2,
                },
            ),
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "-1"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 3,
                },
            ),
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1i32"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 4,
                },
            ),
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "2i64"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 5,
                },
            ),
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "22222222222222222222222222222222222222222222i64"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 6,
                },
            ),
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1 + 1"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 7,
                },
            ),
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "[]i32"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 8,
                },
            ),
            Some(
                3,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "[3]i32"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 9,
                },
            ),
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "(i32, i32)"));

    // expect_test::expect![[r#"
    //     (
    //         ExprRegion {
    //             expr_arena: Arena {
    //                 data: [
    //                     Unrecognized(
    //                         Ident(
    //                             Word(
    //                                 Id {
    //                                     value: 39,
    //                                 },
    //                             ),
    //                         ),
    //                     ),
    //                     Opn {
    //                         opn: List {
    //                             opr: NewLambdaHead,
    //                             bracket: Vertical,
    //                             bra_token_idx: TokenIdx(
    //                                 0,
    //                             ),
    //                             ket_token_idx: TokenIdx(
    //                                 2,
    //                             ),
    //                         },
    //                         opds: ArenaIdxRange(
    //                             0..1,
    //                         ),
    //                     },
    //                     Unrecognized(
    //                         Ident(
    //                             Word(
    //                                 Id {
    //                                     value: 39,
    //                                 },
    //                             ),
    //                         ),
    //                     ),
    //                     Application {
    //                         function: 1,
    //                         argument: 2,
    //                     },
    //                 ],
    //             },
    //             entity_path_expr_arena: Arena {
    //                 data: [],
    //             },
    //             pattern_expr_arena: Arena {
    //                 data: [],
    //             },
    //         },
    //         Some(
    //             3,
    //         ),
    //     )
    // "#]]
    // .assert_debug_eq(&t(&db, "|x|x"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 10,
                },
            ),
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "x.a"));

    expect_test::expect![[r#"
        (
            ExprRegion(
                Id {
                    value: 11,
                },
            ),
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "x.len()"));
}
