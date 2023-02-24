Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Decor {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideModule,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Alien,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Alien`),
                            ),
                        ),
                    ),
                    ident_token: IdentifierToken {
                        ident: `r32`,
                        token_idx: TokenIdx(
                            22,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        23,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
    },
)