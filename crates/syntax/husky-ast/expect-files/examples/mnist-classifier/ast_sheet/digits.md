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
                Defn {
                    token_group_idx: TokenGroupIdx(
                        3,
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
                            `mnist_classifier::digits::three`,
                        ),
                    ),
                    ident: `three`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
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
                            `mnist_classifier::digits::four`,
                        ),
                    ),
                    ident: `four`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        5,
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
                            `mnist_classifier::digits::five`,
                        ),
                    ),
                    ident: `five`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        6,
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
                Defn {
                    token_group_idx: TokenGroupIdx(
                        7,
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
                            `mnist_classifier::digits::seven`,
                        ),
                    ),
                    ident: `seven`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        8,
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
                            `mnist_classifier::digits::eight`,
                        ),
                    ),
                    ident: `eight`,
                    is_generic: false,
                    body_kind: None,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        9,
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
                            `mnist_classifier::digits::nine`,
                        ),
                    ),
                    ident: `nine`,
                    is_generic: false,
                    body_kind: None,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        10,
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
                        11,
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
                        12,
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
            0..13,
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