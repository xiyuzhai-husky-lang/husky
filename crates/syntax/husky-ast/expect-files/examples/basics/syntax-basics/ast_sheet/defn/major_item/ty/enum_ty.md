AstSheet {
    ast_arena: Arena {
        data: [
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 7,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        3,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `UnitVariant`,
                    token_idx: TokenIdx(
                        4,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        5,
                    ),
                    drained: true,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        5,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `PropsVariantWithOneField`,
                    token_idx: TokenIdx(
                        6,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        7,
                    ),
                    drained: false,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 9,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        12,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `PropsVariantWithTwoFields`,
                    token_idx: TokenIdx(
                        13,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        14,
                    ),
                    drained: false,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        23,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `TupleVariantWithOneField`,
                    token_idx: TokenIdx(
                        24,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        25,
                    ),
                    drained: false,
                },
            },
            AstData::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        6,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 11,
                        },
                    ),
                ),
                vertical_token: VerticalToken(
                    TokenIdx(
                        28,
                    ),
                ),
                ident_token: IdentToken {
                    ident: `TupleVariantWithTwoFields`,
                    token_idx: TokenIdx(
                        29,
                    ),
                },
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        30,
                    ),
                    drained: false,
                },
            },
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
                        `syntax_basics::defn::major_item::ty::enum_ty`,
                    ),
                },
                item_kind: EntityKind::MajorItem {
                    module_item_kind: MajorItemKind::Type(
                        TypeKind::Enum,
                    ),
                    connection: MajorItemConnectionKind::Connected,
                },
                ident_token: IdentToken {
                    ident: `A`,
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
                block: DefnBlock::Type {
                    path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    variants: Some(
                        TypeVariants {
                            ast_idx_range: ArenaIdxRange(
                                1..6,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        6..7,
    ),
    nested_top_level_asts: [],
    siblings: [
        ArenaIdxRange(
            6..7,
        ),
    ],
}