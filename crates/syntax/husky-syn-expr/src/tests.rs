pub(crate) use husky_vfs::*;

use crate::*;
use husky_ast::AstJar;
use husky_corgi_config::CorgiConfigJar;
use husky_corgi_config_ast::CorgiConfigAstJar;
use husky_coword::CowordJar;
use husky_entity_path::EntityPathJar;
use husky_entity_syn_tree::*;
use husky_manifest::ManifestJar;
use husky_manifest_ast::ManifestAstJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_vfs::snippet::Snippet;

#[salsa::db(
    CowordJar,
    VfsJar,
    EntityPathJar,
    TermPreludeJar,
    husky_token_data::db::TokenDataJar,
    TokenJar,
    AstJar,
    EntitySynTreeJar,
    TomlTokenJar,
    TomlAstJar,
    ManifestAstJar,
    CorgiConfigJar,
    CorgiConfigAstJar,
    ManifestJar,
    SynExprJar
)]
pub(crate) struct DB;

pub(crate) fn t<'a>(
    db: &'a ::salsa::Db,
    input: &str,
) -> (&'a SynExprRegionData, Option<SynExprIdx>) {
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
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                1,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                2,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        Prefix {
                            opr: Minus,
                            opr_regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                            opd: 1,
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 2,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "-1"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                1,
                            ),
                            Integer(
                                I32(
                                    1,
                                ),
                            ),
                        ),
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1i32"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                1,
                            ),
                            Integer(
                                I64(
                                    2,
                                ),
                            ),
                        ),
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "2i64"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Err(
                            Derived(
                                TokenData(
                                    ParseIntError,
                                ),
                            ),
                        ),
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 1,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                1,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "22222222222222222222222222222222222222222222i64"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                1,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        Literal(
                            RegionalTokenIdx(
                                3,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        Binary {
                            lopd: 1,
                            opr: Closed(
                                Add,
                            ),
                            opr_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ropd: 2,
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 3,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                3,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "1 + 1"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                            items: [],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                        },
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    regional_token_idx: RegionalTokenIdx(
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
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 3,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                3,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "[]i32"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                2,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    3,
                                ),
                            ),
                        ),
                        List {
                            lbox_regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 1,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rbox_regional_token_idx: RegionalTokenIdx(
                                3,
                            ),
                        },
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
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
                            function_expr_idx: 2,
                            argument_expr_idx: 3,
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 4,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                4,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "[3]i32"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    regional_token_idx: RegionalTokenIdx(
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
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
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
                            lpar_regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                            items: [
                                SynCommaListItem {
                                    syn_expr_idx: 1,
                                    comma_regional_token_idx: Some(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                },
                                SynCommaListItem {
                                    syn_expr_idx: 2,
                                    comma_regional_token_idx: None,
                                },
                            ],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 3,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                3,
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
    //                             bra_token_idx: RegionalTokenIdx(
    //                                 0,
    //                             ),
    //                             ket_token_idx: RegionalTokenIdx(
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
    //             item_path_expr_arena: Arena {
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
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 69,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        Field {
                            owner: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 70,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 2,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                2,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "x.a"));

    expect_test::expect![[r#"
        (
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Err(
                            Original(
                                UnrecognizedIdent {
                                    regional_token_idx: RegionalTokenIdx(
                                        1,
                                    ),
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 69,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        MethodApplicationOrCall {
                            self_argument: 1,
                            dot_regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                            ident_token: IdentRegionalToken {
                                ident: Ident(
                                    Coword(
                                        Id {
                                            value: 71,
                                        },
                                    ),
                                ),
                                regional_token_idx: RegionalTokenIdx(
                                    3,
                                ),
                            },
                            template_arguments: None,
                            lpar_regional_token_idx: RegionalTokenIdx(
                                4,
                            ),
                            items: [],
                            rpar_regional_token_idx: RegionalTokenIdx(
                                5,
                            ),
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 2,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                2,
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
            SynExprRegionData {
                parent: None,
                path: Snippet(
                    ModulePath(
                        Id {
                            value: 1,
                        },
                    ),
                ),
                syn_expr_arena: Arena {
                    data: [
                        Literal(
                            RegionalTokenIdx(
                                1,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    0,
                                ),
                            ),
                        ),
                        Literal(
                            RegionalTokenIdx(
                                2,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    1,
                                ),
                            ),
                        ),
                        ExplicitApplication {
                            function_expr_idx: 1,
                            argument_expr_idx: 2,
                        },
                        Literal(
                            RegionalTokenIdx(
                                3,
                            ),
                            Integer(
                                UnspecifiedRegular(
                                    2,
                                ),
                            ),
                        ),
                        ExplicitApplication {
                            function_expr_idx: 3,
                            argument_expr_idx: 4,
                        },
                    ],
                },
                principal_item_path_expr_arena: Arena {
                    data: [],
                },
                stmt_arena: Arena {
                    data: [],
                },
                pattern_expr_region: SynPatternExprRegion {
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    pattern_expr_contracts: ArenaMap {
                        data: [],
                    },
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
                symbol_region: SynSymbolRegionData {
                    inherited_syn_symbol_arena: Arena {
                        data: [],
                    },
                    current_syn_symbol_arena: Arena {
                        data: [],
                    },
                    allow_self_type: False,
                    allow_self_value: False,
                    pattern_ty_constraints: [],
                },
                syn_pattern_expr_roots: [],
                syn_expr_roots: [
                    SynExprRoot {
                        kind: Snippet,
                        syn_expr_idx: 5,
                    },
                ],
                has_self_lifetime: false,
                has_self_place: false,
                syn_pattern_to_current_syn_symbol_map: [],
            },
            Some(
                5,
            ),
        )
    "#]]
    .assert_debug_eq(&t(&db, "0 1 2"));
}
