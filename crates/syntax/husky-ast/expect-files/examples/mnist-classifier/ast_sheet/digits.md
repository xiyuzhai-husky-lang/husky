Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    entity_kind: Module,
                    entity_path: Some(
                        Module(
                            `mnist_classifier::digits::zero`,
                        ),
                    ),
                    ident: `zero`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    entity_kind: Module,
                    entity_path: Some(
                        Module(
                            `mnist_classifier::digits::one`,
                        ),
                    ),
                    ident: `one`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    entity_kind: Module,
                    entity_path: Some(
                        Module(
                            `mnist_classifier::digits::six`,
                        ),
                    ),
                    ident: `six`,
                    is_generic: false,
                    body_kind: None,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    ident: `one`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    use_expr_idx: 1,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    ident: `six`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    use_expr_idx: 3,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    ident: `zero`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..6,
        ),
        use_expr_arena: Arena {
            data: [
                One {
                    ident: `is_one`,
                },
                ScopeResolution {
                    parent: `one`,
                    child: 0,
                },
                One {
                    ident: `is_six`,
                },
                ScopeResolution {
                    parent: `six`,
                    child: 2,
                },
                One {
                    ident: `is_zero`,
                },
                ScopeResolution {
                    parent: `zero`,
                    child: 4,
                },
            ],
        },
    },
)