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
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_basics::uses`,
                    ),
                },
                state_after_visibility_expr: None,
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..2,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            1..2,
        ),
    ],
}
```