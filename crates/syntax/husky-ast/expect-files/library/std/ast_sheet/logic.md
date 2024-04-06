```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                1,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                state_after_visibility_expr: Some(
                    TokenStreamState {
                        next_token_idx: TokenIdx(
                            2,
                        ),
                        drained: false,
                    },
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        0..1,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            0..1,
        ),
    ],
}
```