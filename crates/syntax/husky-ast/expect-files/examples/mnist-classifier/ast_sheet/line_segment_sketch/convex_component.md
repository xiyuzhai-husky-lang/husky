Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    error: Original(
                        ExpectIdentifier(
                            TokenIdx(
                                26,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: Original(
                        ExcessiveIndent,
                    ),
                },
                Ast::Use {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Struct,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            ),
                        ),
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
                Ast::Impl {
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