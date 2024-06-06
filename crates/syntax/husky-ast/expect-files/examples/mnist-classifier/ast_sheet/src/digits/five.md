```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 2,
                },
                body: None,
            },
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`mnist_classifier::digits::five`),
                    ),
                },
                state_after_visibility_expr: None,
            },
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 1,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::PubUnder {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                6,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                8,
                            ),
                        ),
                    },
                    visibility: Scope::PubUnder(
                        ModulePath(`mnist_classifier::digits`),
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Form(
                        MajorFormKind::Val,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `is_five`,
                    token_idx: TokenIdx(
                        10,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        11,
                    ),
                    drained: false,
                },
                block: DefnBlock::Form {
                    path: FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                    body: Some(
                        FormBody {
                            ast_idx_range: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..3,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..0,
        ),
        ArenaIdxRange(
            0..1,
        ),
        ArenaIdxRange(
            1..3,
        ),
    ],
}
```