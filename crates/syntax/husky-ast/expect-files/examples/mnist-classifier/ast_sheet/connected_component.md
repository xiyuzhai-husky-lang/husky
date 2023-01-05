Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        2..2,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    error: ExpectNothing,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        12,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        13,
                    ),
                    error: ExpectNothing,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        14,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        15,
                    ),
                    error: ExcessiveIndent,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `cv`,
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
                        1,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    use_expr_idx: 6,
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
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `ConnectedComponentDistribution`,
                        token_idx: TokenIdx(
                            15,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        16,
                    ),
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
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `EffHoles`,
                        token_idx: TokenIdx(
                            35,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        36,
                    ),
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Form(
                            Function,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `hole_tmpl`,
                        token_idx: TokenIdx(
                            47,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        48,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    error: ExcessiveIndent,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        2..8,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `ConnectedComponent`,
                        token_idx: TokenIdx(
                            72,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        73,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        17,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        18,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    error: ExcessiveIndent,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            8..20,
        ),
        use_expr_arena: Arena {
            data: [
                One {
                    ident: `BinaryImage28`,
                },
                ScopeResolution {
                    parent: `mnist`,
                    child: 0,
                },
                ScopeResolution {
                    parent: `datasets`,
                    child: 1,
                },
                ScopeResolution {
                    parent: `cv`,
                    child: 2,
                },
                All,
                ScopeResolution {
                    parent: `raw_contour`,
                    child: 4,
                },
                ScopeResolution {
                    parent: `crate`,
                    child: 5,
                },
            ],
        },
    },
)