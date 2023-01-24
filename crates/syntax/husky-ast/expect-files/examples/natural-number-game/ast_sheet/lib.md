Ok(
    AstSheet {
        arena: Arena {
            data: [
                Defn {
                    token_group_idx: TokenGroupIdx(
                        4,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 40,
                            },
                        ),
                    ),
                    entity_kind: AssociatedItem {
                        associated_item_kind: TypeItem(
                            Memo,
                        ),
                    },
                    entity_path: None,
                    ident_token: IdentifierToken {
                        ident: `add`,
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    is_generic: false,
                    body_kind: EnumVariants,
                    saved_stream_state: TokenIdx(
                        14,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        5,
                    ),
                    error: UnexpectedStmtInsideImpl,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        6,
                    ),
                    error: UnexpectedStmtInsideImpl,
                },
                Defn {
                    token_group_idx: TokenGroupIdx(
                        0,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 40,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Inductive,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`natural_number_game::Nat`, `Inductive`),
                    ),
                    ident_token: IdentifierToken {
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
                Err {
                    token_group_idx: TokenGroupIdx(
                        1,
                    ),
                    error: UnexpectedStmtInsideModule,
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        2,
                    ),
                    error: UnexpectedStmtInsideModule,
                },
                Impl {
                    token_group_idx: TokenGroupIdx(
                        3,
                    ),
                    body: ArenaIdxRange(
                        0..3,
                    ),
                },
                Err {
                    token_group_idx: TokenGroupIdx(
                        7,
                    ),
                    error: ExpectIdentifier(
                        TokenIdx(
                            46,
                        ),
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
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 40,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`natural_number_game::OddNat`, `Structure`),
                    ),
                    ident_token: IdentifierToken {
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
                Defn {
                    token_group_idx: TokenGroupIdx(
                        10,
                    ),
                    body: ArenaIdxRange(
                        0..0,
                    ),
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 40,
                            },
                        ),
                    ),
                    entity_kind: ModuleItem {
                        module_item_kind: Type(
                            Structure,
                        ),
                        connection: Connected,
                    },
                    entity_path: Some(
                        TypePath(`natural_number_game::EvenNat`, `Structure`),
                    ),
                    ident_token: IdentifierToken {
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
    },
)