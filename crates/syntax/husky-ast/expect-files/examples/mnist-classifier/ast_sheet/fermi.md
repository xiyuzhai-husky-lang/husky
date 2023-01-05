Ok(
    AstSheet {
        arena: Arena {
            data: [
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        20,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Stmt {
                    token_group_idx: TokenGroupIdx(
                        21,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    ident: `crate`,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                    use_expr_idx: 1,
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
                                value: 25,
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
                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `FermiMatchResult`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        6,
                    ),
                },
                Impl {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        11,
                    ),
                    error: ExcessiveIndent,
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
                    error: ExcessiveIndent,
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
                Err {
                    token_group_idx: TokenGroupIdx(
                        16,
                    ),
                    error: ExcessiveIndent,
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
                Defn {
                    token_group_idx: TokenGroupIdx(
                        19,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 25,
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
                        FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `fermi_match`,
                        token_idx: TokenIdx(
                            134,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        135,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        23,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        24,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        25,
                    ),
                    error: ExcessiveIndent,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        26,
                    ),
                    error: ExcessiveIndent,
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..25,
        ),
        use_expr_arena: Arena {
            data: [
                All,
                ScopeResolution {
                    parent: `crate`,
                    child: 0,
                },
            ],
        },
    },
)