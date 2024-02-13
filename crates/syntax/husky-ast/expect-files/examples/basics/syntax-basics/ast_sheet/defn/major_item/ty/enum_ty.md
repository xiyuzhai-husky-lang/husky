AstSheet {
    ast_arena: Arena {
        data: [
            Ast::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        2,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `UnitVariant`,
                                index: U8(
                                    0,
                                ),
                            },
                        ),
                    },
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
            Ast::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        3,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `PropsVariantWithOneField`,
                                index: U8(
                                    1,
                                ),
                            },
                        ),
                    },
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
            Ast::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        4,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `PropsVariantWithTwoFields`,
                                index: U8(
                                    2,
                                ),
                            },
                        ),
                    },
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
            Ast::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        5,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `TupleVariantWithOneField`,
                                index: U8(
                                    3,
                                ),
                            },
                        ),
                    },
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
            Ast::TypeVariant {
                token_verse_idx: TokenVerseIdx {
                    lcurl: None,
                    raw: ShiftedU32(
                        6,
                    ),
                },
                variant_path: TypeVariantPath(
                    ItemPathId {
                        data: ItemPathData::TypeVariant(
                            TypeVariantPathData {
                                parent_ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                                ident: `TupleVariantWithTwoFields`,
                                index: U8(
                                    4,
                                ),
                            },
                        ),
                    },
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
            Ast::Identifiable {
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
                item_kind: MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
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
    siblings: [
        ArenaIdxRange(
            6..7,
        ),
    ],
}