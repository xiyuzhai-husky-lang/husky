EntitySynTreeCrateBundle {
    sheets: [
        EntitySynTreeSheet {
            module_path: `malamute`,
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
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::Class`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 13,
                                ident_token: IdentToken {
                                    ident: `Class`,
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::Class`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                1..3,
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
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::Class`, `Enum`),
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
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 15,
                                ident_token: IdentToken {
                                    ident: `OneVsAll`,
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                3..5,
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
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 19,
                                ident_token: IdentToken {
                                    ident: `OneVsAllResult`,
                                    token_idx: TokenIdx(
                                        120,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                8..11,
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
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                syn_node_path: MajorItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 21,
                                ident_token: IdentToken {
                                    ident: `narrow_down`,
                                    token_idx: TokenIdx(
                                        172,
                                    ),
                                },
                                block: DefnBlock::Fugitive {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Fugitive(
                                                FugitiveSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
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
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAll`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAllResult`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `narrow_down`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
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
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::default::Default`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
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
                                                path: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::default::Default`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 16,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                            trai_expr: 1,
                            for_token: TokenIdx(
                                65,
                            ),
                            ty_sketch_expr: Path(
                                2,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            6..7,
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
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::Class`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
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
                                                path: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::Class`, `Enum`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 17,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    81,
                                ),
                            },
                            trai_expr: 5,
                            for_token: TokenIdx(
                                100,
                            ),
                            ty_sketch_expr: Path(
                                6,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            7..8,
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
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `malamute`,
                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
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
                                                path: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `malamute`,
                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 20,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    139,
                                ),
                            },
                            trai_expr: 9,
                            for_token: TokenIdx(
                                160,
                            ),
                            ty_sketch_expr: Path(
                                10,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            11..12,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ],
            once_use_rules: UseOneRules(
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
                            64,
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
                            66,
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
                            96,
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
                            94,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        95,
                    ),
                ),
                subexpr: 3,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            92,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        93,
                    ),
                ),
                subexpr: 4,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Class`,
                        token_idx: TokenIdx(
                            101,
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
                            156,
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
                            154,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        155,
                    ),
                ),
                subexpr: 7,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            152,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        153,
                    ),
                ),
                subexpr: 8,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `OneVsAll`,
                        token_idx: TokenIdx(
                            161,
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