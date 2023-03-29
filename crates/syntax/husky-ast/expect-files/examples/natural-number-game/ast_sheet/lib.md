Ok(
    AstSheet {
        ast_arena: Arena {
            data: [
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    error: AstError::Original(
                        OriginalAstError::UnexpectedStmtInsideImplBlock,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `natural_number_game`,
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`natural_number_game::Nat`, `Inductive`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `Nat`,
                        token_idx: TokenIdx(
                            1,
                        ),
                    },
                    is_generic: false,
                    body_kind: EnumVariants,
                    saved_stream_state: TokenIdx(
                        2,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::BasicStmtOrBranch {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                },
                Ast::Impl {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..3,
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExpectedIdent(
                            TokenIdx(
                                46,
                            ),
                        ),
                    ),
                },
                Ast::Err {
                    token_group_idx: TokenGroupIdx(
                        8,
                    ),
                    error: AstError::Original(
                        OriginalAstError::ExcessiveIndent,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        9,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `natural_number_game`,
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`natural_number_game::OddNat`, `Structure`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `OddNat`,
                        token_idx: TokenIdx(
                            85,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        86,
                    ),
                },
                Ast::Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: Visibility::PublicUnder(
                        `natural_number_game`,
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        EntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`natural_number_game::EvenNat`, `Structure`),
                            ),
                        ),
                    ),
                    ident_token: IdentToken {
                        ident: `EvenNat`,
                        token_idx: TokenIdx(
                            114,
                        ),
                    },
                    is_generic: false,
                    body_kind: None,
                    saved_stream_state: TokenIdx(
                        115,
                    ),
                },
            ],
        },
        top_level_asts: ArenaIdxRange(
            3..11,
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
            ArenaIdxRange(
                3..3,
            ),
            ArenaIdxRange(
                3..3,
            ),
            ArenaIdxRange(
                3..11,
            ),
        ],
    },
)