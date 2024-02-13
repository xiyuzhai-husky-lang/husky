AstSheet {
    ast_arena: Arena {
        data: [
            Ast::BasicStmtOrBranch {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
                },
                body: None,
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                },
                item_kind: AssocItem {
                    assoc_item_kind: TraitForTypeItem(
                        MethodRitchie(
                            Fn,
                        ),
                    ),
                },
                ident_token: IdentToken {
                    ident: `visualize`,
                    token_idx: TokenIdx(
                        28,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        29,
                    ),
                    drained: false,
                },
                block: DefnBlock::AssocItem {
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                },
            },
            Ast::Use {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        1,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Identifiable {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                },
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Struct,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `ConvexComponent`,
                    token_idx: TokenIdx(
                        8,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        9,
                    ),
                    drained: false,
                },
                block: DefnBlock::Type {
                    path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                    variants: None,
                },
            },
            Ast::ImplBlock {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                items: Some(
                    TraitForType(
                        TraitForTypeItems {
                            ast_idx_range: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        3..6,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            2..3,
        ),
        ArenaIdxRange(
            3..6,
        ),
    ],
}