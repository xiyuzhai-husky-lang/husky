Ok(
    AstSheet {
        arena: Arena {
            data: [
                Err {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    error: ExpectNothing,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    error: ExpectNothing,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: ExpectIdentifier(
                        TokenIdx(
                            20,
                        ),
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        5,
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
                        0..4,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 29,
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
                        TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent`, `Struct`),
                    ),
                    ident_token: IdentifierToken {
                        ident: `ConvexCompoent`,
                        token_idx: TokenIdx(
                            7,
                        ),
                    },
                    is_generic: false,
                    body_kind: Block,
                    saved_stream_state: TokenIdx(
                        8,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            4..6,
        ),
    },
)