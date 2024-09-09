```rust
AstSheet {
    ast_arena: Arena {
        data: [
            AstData::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: 0,
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        ModulePath(`syntax_basics::ast`),
                    ),
                },
                item_kind: EntityKind::Module,
                ident_token: IdentToken {
                    ident: `submodule_name`,
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
                    path: SubmoduleItemPath(`syntax_basics::ast::submodule_name),
                },
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