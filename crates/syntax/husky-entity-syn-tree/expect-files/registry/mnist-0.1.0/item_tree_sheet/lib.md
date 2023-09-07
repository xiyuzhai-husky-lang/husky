Ok(
    EntitySynTreeSheet {
        module_path: `mnist`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 16,
                            ident_token: IdentToken {
                                ident: `MnistLabel`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 73,
                                    },
                                ),
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
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::MnistLabel`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `MnistLabel`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 18,
                            ident_token: IdentToken {
                                ident: `BinaryImage28`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 74,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `BinaryImage28`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `BinaryGrid28`,
                                token_idx: TokenIdx(
                                    79,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 75,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `BinaryGrid28`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist::input`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 26,
                            ident_token: IdentToken {
                                ident: `input`,
                                token_idx: TokenIdx(
                                    125,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                body: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist::input`, `Val`),
                                    disambiguator: 0,
                                },
                            },
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
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`mnist::MnistLabel`, `Enum`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::MnistLabel`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 16,
                            ident_token: IdentToken {
                                ident: `MnistLabel`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 73,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            0..10,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `BinaryImage28`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`mnist::BinaryImage28`, `Struct`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 18,
                            ident_token: IdentToken {
                                ident: `BinaryImage28`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 74,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `BinaryGrid28`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `BinaryGrid28`,
                                token_idx: TokenIdx(
                                    79,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 75,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `input`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist::input`, `Val`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist::input`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 26,
                            ident_token: IdentToken {
                                ident: `input`,
                                token_idx: TokenIdx(
                                    125,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 81,
                                    },
                                ),
                                body: None,
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_syn_node_table: [
            (
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryImage28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        syn_node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 19,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                40,
                            ),
                        },
                        trai_expr: 0,
                        for_token: TokenIdx(
                            42,
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
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist`,
                            ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TypeImplBlock(
                    TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 20,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                52,
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
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryImage28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        syn_node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::ops::IntIndex`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 21,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                64,
                            ),
                        },
                        trai_expr: 5,
                        for_token: TokenIdx(
                            70,
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
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryGrid28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        syn_node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 23,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                86,
                            ),
                        },
                        trai_expr: 7,
                        for_token: TokenIdx(
                            88,
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
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist`,
                            ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TypeImplBlock(
                    TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 24,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                98,
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
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist`,
                            trai_path: TraitPath(`core::ops::IntIndex`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist::BinaryGrid28`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        syn_node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist`,
                                trai_path: TraitPath(`core::ops::IntIndex`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 25,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                110,
                            ),
                        },
                        trai_expr: 12,
                        for_token: TokenIdx(
                            116,
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
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [],
    },
)