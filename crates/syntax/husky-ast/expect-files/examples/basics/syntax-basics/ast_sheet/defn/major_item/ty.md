```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `syntax_basics::defn::major_item::ty`,
                    ),
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `enum_ty`,
                    token_idx: TokenIdx(
                        2,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        3,
                    ),
                    drained: true,
                },
                block: DefnBlock::Submodule {
                    path: SubmoduleItemPath(
                        ItemPathId(
                            Id {
                                value: 12,
                            },
                        ),
                    ),
                },
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