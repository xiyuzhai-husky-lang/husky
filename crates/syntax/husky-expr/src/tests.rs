pub(crate) use husky_vfs::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_entity_tree::*;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::*;

#[salsa::db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    TermPreludeJar,
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

pub(crate) fn t<'a>(db: &'a DB, input: &str) -> (&'a ExprRegionData, Option<ExprIdx>) {
    let toolchain = db.dev_toolchain().unwrap();
    let path_menu = db.vfs_path_menu(toolchain);
    let snippet = Snippet::new(db, input.to_owned());
    let (expr_region, expr_idx) = parse_expr_from_snippet(db, path_menu.core_library(), snippet)
        .as_ref()
        .unwrap();
    (expr_region.data(db), *expr_idx)
}

#[test]
fn parse_expr_works() {
    let db = DB::default();
    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 0,
                    },
                ],
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                1,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        Prefix {
                            opr: Minus,
                            opr_token_idx: TokenIdx(
                                0,
                            ),
                            opd: 0,
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 1,
                    },
                ],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "-1"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                            Integer(
                                I32(
                                    1,
                                ),
                            ),
                        ),
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 0,
                    },
                ],
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1i32"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                            Integer(
                                I64(
                                    2,
                                ),
                            ),
                        ),
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 0,
                    },
                ],
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "2i64"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Err(
                            Derived(
                                Token(
                                    ParseIntError,
                                ),
                            ),
                        ),
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 0,
                    },
                ],
            },
            Some(
                0,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "22222222222222222222222222222222222222222222i64"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        Literal(
                            TokenIdx(
                                2,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        Binary {
                            lopd: 0,
                            opr: Closed(
                                Add,
                            ),
                            opr_token_idx: TokenIdx(
                                1,
                            ),
                            ropd: 1,
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 2,
                    },
                ],
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1 + 1"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        List {
                            lbox_token_idx: TokenIdx(
                                0,
                            ),
                            items: [],
                            rbox_token_idx: TokenIdx(
                                1,
                            ),
                        },
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    token_idx: TokenIdx(
                                        2,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        ExplicitApplication {
                            function_expr_idx: 0,
                            argument_expr_idx: 1,
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 2,
                    },
                ],
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "[]i32"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                1,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    3,
                                ),
                            ),
                        ),
                        List {
                            lbox_token_idx: TokenIdx(
                                0,
                            ),
                            items: [
                                CommaListItem {
                                    expr_idx: 0,
                                    comma_token_idx: None,
                                },
                            ],
                            rbox_token_idx: TokenIdx(
                                2,
                            ),
                        },
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        ExplicitApplication {
                            function_expr_idx: 1,
                            argument_expr_idx: 2,
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 3,
                    },
                ],
            },
            Some(
                3,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "[3]i32"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    token_idx: TokenIdx(
                                        1,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 9,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        NewTuple {
                            lpar_token_idx: TokenIdx(
                                0,
                            ),
                            items: [
                                CommaListItem {
                                    expr_idx: 0,
                                    comma_token_idx: Some(
                                        TokenIdx(
                                            2,
                                        ),
                                    ),
                                },
                                CommaListItem {
                                    expr_idx: 1,
                                    comma_token_idx: None,
                                },
                            ],
                            rpar_token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 2,
                    },
                ],
            },
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
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    token_idx: TokenIdx(
                                        0,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Field {
                            owner: 0,
                            dot_token_idx: TokenIdx(
                                1,
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 67,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 1,
                    },
                ],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "x.a"));

    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    token_idx: TokenIdx(
                                        0,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 66,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        MethodApplicationOrCall {
                            self_argument: 0,
                            dot_token_idx: TokenIdx(
                                1,
                            ),
                            ident_token: IdentToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 68,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            implicit_arguments: None,
                            lpar_token_idx: TokenIdx(
                                3,
                            ),
                            items: [],
                            rpar_token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 1,
                    },
                ],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "x.len()"));
}

#[test]
fn parse_application_expr_works() {
    let db = DB::default();
    // this is wrong semantically, but useful for specifying syntactic behavior
    expect_test::expect![[r#"
        (
            ExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                expr_arena: Arena {
                    data: [
                        Literal(
                            TokenIdx(
                                0,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        Literal(
                            TokenIdx(
                                1,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        ExplicitApplication {
                            function_expr_idx: 0,
                            argument_expr_idx: 1,
                        },
                        Literal(
                            TokenIdx(
                                2,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    2,
                                ),
                            ),
                        ),
                        ExplicitApplication {
                            function_expr_idx: 2,
                            argument_expr_idx: 3,
                        },
                    ],
                },
                principal_entity_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: PatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
                    pattern_infos: [],
                    pattern_symbol_arena: Arena {
                        data: [],
                    },
                    pattern_symbol_maps: ArenaMap {
                        data: [],
                    },
                    pattern_symbol_modifiers: ArenaMap {
                        data: [],
                    },
                },
                symbol_region: SymbolRegion {
                    inherited_symbol_arena: Arena {
                        data: [],
                    },
                    current_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                roots: [
                    ExprRoot {
                        kind: Snippet,
                        expr_idx: 4,
                    },
                ],
            },
            Some(
                4,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "0 1 2"));
}
