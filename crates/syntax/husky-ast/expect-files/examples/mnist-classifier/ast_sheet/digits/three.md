Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `domains`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    ident: `domains`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 10,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    ident: `domains`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 15,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    ident: `domains`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 20,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 23,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 26,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    use_expr_idx: 30,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    body: ArenaIdxRange(
                        0..1,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Feature,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `three_fermi_match`,
                        token_idx: TokenIdx(
                            63,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        64,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            1..9,
        ),
        use_expr_arena: Arena {
            data: [
                One {
                    ident: `MnistLabel`,
                },
                ScopeResolution {
                    parent: `mnist`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `cv`,
                    child: 1,
                },
                ScopeResolution {
                    parent: `datasets`,
                    child: 2,
                },
                ScopeResolution {
                    parent: `ml`,
                    child: 3,
                },
                ScopeResolution {
                    parent: `domains`,
                    child: 4,
                },
                One {
                    ident: `normalize_vmax_f32`,
                },
                ScopeResolution {
                    parent: `normalize`,
                    child: 6,
                },
                ScopeResolution {
                    parent: `models`,
                    child: 7,
                },
                ScopeResolution {
                    parent: `ml`,
                    child: 8,
                },
                ScopeResolution {
                    parent: `domains`,
                    child: 9,
                },
                One {
                    ident: `boosting_with_vmax_normalized`,
                },
                ScopeResolution {
                    parent: `boosting`,
                    child: 11,
                },
                ScopeResolution {
                    parent: `models`,
                    child: 12,
                },
                ScopeResolution {
                    parent: `ml`,
                    child: 13,
                },
                ScopeResolution {
                    parent: `domains`,
                    child: 14,
                },
                One {
                    ident: `narrow_down`,
                },
                ScopeResolution {
                    parent: `narrow`,
                    child: 16,
                },
                ScopeResolution {
                    parent: `models`,
                    child: 17,
                },
                ScopeResolution {
                    parent: `ml`,
                    child: 18,
                },
                ScopeResolution {
                    parent: `domains`,
                    child: 19,
                },
                All,
                ScopeResolution {
                    parent: `major`,
                    child: 21,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 22,
                },
                All,
                ScopeResolution {
                    parent: `fermi`,
                    child: 24,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 25,
                },
                All,
                ScopeResolution {
                    parent: `concave_component`,
                    child: 27,
                },
                ScopeResolution {
                    parent: `line_segment_sketch`,
                    child: 28,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 29,
                },
            ],
        },
    },
)