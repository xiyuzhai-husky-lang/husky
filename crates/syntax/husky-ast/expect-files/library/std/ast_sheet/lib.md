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
                    accessibility: Visibility::PublicUnder(
                        `std`,
                    ),
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `std::prelude`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `prelude`,
                        token_idx: TokenIdx(
                            1,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        2,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `std`,
                    ),
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `std::logic`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `logic`,
                        token_idx: TokenIdx(
                            3,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        4,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `std`,
                    ),
                    entity_kind: Module,
                    entity_path: Some(
                        EntityPath::Module(
                            `std::ops`,
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `ops`,
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
            ],
        },
        top_level_asts: ArenaIdxRange(
            0..3,
        ),
        siblings: [
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..0,
            ),
            ArenaIdxRange(
                0..3,
            ),
        ],
    },
)