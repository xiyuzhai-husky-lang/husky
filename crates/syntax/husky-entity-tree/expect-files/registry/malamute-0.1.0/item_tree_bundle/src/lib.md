```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`malamute`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypePath(`malamute::Class`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 24,
                                ident_token: IdentToken {
                                    ident: `Class`,
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::Class`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                0..2,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`malamute::Class`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `Class`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 26,
                                ident_token: IdentToken {
                                    ident: `OneVsAll`,
                                    token_idx: TokenIdx(
                                        46,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                2..4,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `OneVsAll`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 30,
                                ident_token: IdentToken {
                                    ident: `OneVsAllResult`,
                                    token_idx: TokenIdx(
                                        191,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                12..15,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `OneVsAllResult`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`, (0)),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 33,
                                ident_token: IdentToken {
                                    ident: `narrow_down`,
                                    token_idx: TokenIdx(
                                        321,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`, (0)),
                            ),
                        ),
                        ident: `narrow_down`,
                        visibility: Scope::Pub,
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `Class`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAll`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAllResult`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `narrow_down`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNode {
                            syn_node_path: TraitForTypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePathData {
                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 27,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    63,
                                ),
                            },
                            trai_expr: 0,
                            for_token: TokenIdx(
                                77,
                            ),
                            ty_sketch_expr: Path(
                                1,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            5..6,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNode {
                            syn_node_path: TraitForTypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePathData {
                                                path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 28,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    93,
                                ),
                            },
                            trai_expr: 4,
                            for_token: TokenIdx(
                                112,
                            ),
                            ty_sketch_expr: Path(
                                5,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            10..12,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNode {
                            syn_node_path: TraitForTypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePathData {
                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 31,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    210,
                                ),
                            },
                            trai_expr: 8,
                            for_token: TokenIdx(
                                231,
                            ),
                            ty_sketch_expr: Path(
                                9,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            20..22,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Default`,
                        token_idx: TokenIdx(
                            76,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::default::Default`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `OneVsAll`,
                        token_idx: TokenIdx(
                            78,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Unveil`,
                        token_idx: TokenIdx(
                            108,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            106,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        107,
                    ),
                ),
                subexpr: 2,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            104,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        105,
                    ),
                ),
                subexpr: 3,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Class`,
                        token_idx: TokenIdx(
                            113,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Unveil`,
                        token_idx: TokenIdx(
                            227,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            225,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        226,
                    ),
                ),
                subexpr: 6,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            223,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        224,
                    ),
                ),
                subexpr: 7,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `OneVsAll`,
                        token_idx: TokenIdx(
                            232,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            },
        ],
    },
}
```