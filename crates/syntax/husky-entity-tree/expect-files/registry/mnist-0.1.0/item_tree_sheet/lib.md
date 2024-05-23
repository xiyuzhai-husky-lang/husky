```rust
EntityTreeSheet {
    module_path: `mnist`,
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 16,
                        ident_token: IdentToken {
                            ident: `MnistLabel`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::MnistLabel`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        0..10,
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
                                                maybe_ambiguous_item_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `MnistLabel`,
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 18,
                        ident_token: IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                33,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::BinaryImage28`, `Extern`),
                            variants: None,
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
                                                maybe_ambiguous_item_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `BinaryImage28`,
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 22,
                        ident_token: IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                74,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                            variants: None,
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
                                                maybe_ambiguous_item_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `BinaryGrid28`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`mnist::input`, `Val`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 26,
                        ident_token: IdentToken {
                            ident: `input`,
                            token_idx: TokenIdx(
                                115,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist::input`, `Val`),
                            body: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        FormSynNodePath(`mnist::input`, `Val`, (0)),
                    ),
                ),
                ident: `input`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `MnistLabel`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `BinaryImage28`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Extern`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `BinaryGrid28`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `input`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist::input`, `Val`),
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
                                    path: TraitForTypeImplBlockPath(`mnist::BinaryImage28 as core::visual::Visualize(0)`),
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
                                        path: TraitForTypeImplBlockPath(`mnist::BinaryImage28 as core::visual::Visualize(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 19,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            35,
                        ),
                    },
                    trai_expr: 0,
                    for_token: TokenIdx(
                        37,
                    ),
                    ty_sketch_expr: Path(
                        1,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    10..11,
                                ),
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(`mnist::BinaryImage28(0)`),
                                },
                            ),
                        ),
                    },
                ),
            ),
            ImplBlockSynNode::TypeImplBlock(
                TypeImplBlockSynNode {
                    syn_node_path: TypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TypeImplBlock(
                                    TypeImplBlockSynNodePathData {
                                        path: TypeImplBlockPath(`mnist::BinaryImage28(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 20,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            47,
                        ),
                    },
                    ty_expr: 2,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            11..12,
                        ),
                    },
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
                                    path: TraitForTypeImplBlockPath(`mnist::BinaryImage28 as core::ops::IntIndex(0)`),
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
                                        path: TraitForTypeImplBlockPath(`mnist::BinaryImage28 as core::ops::IntIndex(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 21,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            59,
                        ),
                    },
                    trai_expr: 5,
                    for_token: TokenIdx(
                        65,
                    ),
                    ty_sketch_expr: Path(
                        6,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    12..13,
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
                                    path: TraitForTypeImplBlockPath(`mnist::BinaryGrid28 as core::visual::Visualize(0)`),
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
                                        path: TraitForTypeImplBlockPath(`mnist::BinaryGrid28 as core::visual::Visualize(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 23,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            76,
                        ),
                    },
                    trai_expr: 7,
                    for_token: TokenIdx(
                        78,
                    ),
                    ty_sketch_expr: Path(
                        8,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    13..14,
                                ),
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(`mnist::BinaryGrid28(0)`),
                                },
                            ),
                        ),
                    },
                ),
            ),
            ImplBlockSynNode::TypeImplBlock(
                TypeImplBlockSynNode {
                    syn_node_path: TypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TypeImplBlock(
                                    TypeImplBlockSynNodePathData {
                                        path: TypeImplBlockPath(`mnist::BinaryGrid28(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 24,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            88,
                        ),
                    },
                    ty_expr: 9,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            14..15,
                        ),
                    },
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
                                    path: TraitForTypeImplBlockPath(`mnist::BinaryGrid28 as core::ops::IntIndex(0)`),
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
                                        path: TraitForTypeImplBlockPath(`mnist::BinaryGrid28 as core::ops::IntIndex(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 25,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            100,
                        ),
                    },
                    trai_expr: 12,
                    for_token: TokenIdx(
                        106,
                    ),
                    ty_sketch_expr: Path(
                        13,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    15..16,
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
}
```