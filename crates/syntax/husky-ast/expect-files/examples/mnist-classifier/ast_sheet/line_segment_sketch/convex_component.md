Ok(
    AstSheet {
        arena: Arena {
            data: [
                Err {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    error: ExpectIdentifier(
                        TokenIdx(
                            26,
                        ),
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: ExcessiveIndent,
                },
                Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
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
                                value: 36,
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
                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `ConvexComponent`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        8,
                    ),
                },
                Impl {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..2,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            2..5,
        ),
    },
)