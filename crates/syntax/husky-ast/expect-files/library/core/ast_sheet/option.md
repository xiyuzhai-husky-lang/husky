Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Accessibility::Public,
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Enum,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Option`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    body_kind: EnumVariants,
                    saved_stream_state: TokenIdx(
                        3,
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
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideModule,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
    },
)