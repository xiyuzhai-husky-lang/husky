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
                            Extern,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::list::List`, `Extern`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `List`,
                        token_idx: TokenIdx(
                            2,
                        ),
                    },
                    is_generic: true,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        3,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..1,
        ),
    },
)