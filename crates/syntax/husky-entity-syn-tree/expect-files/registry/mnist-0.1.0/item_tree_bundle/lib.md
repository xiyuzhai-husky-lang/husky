Ok(
    EntitySynTreeCrateBundle {
        sheets: [
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
                                            2,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 71,
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
                                            32,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 72,
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
                                            78,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 73,
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
                                            124,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 80,
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
                                            2,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 71,
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
                                            32,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 72,
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
                                            78,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 73,
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
                                            124,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 80,
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
                                        39,
                                    ),
                                },
                                trai_expr: 0,
                                for_token: TokenIdx(
                                    41,
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
                                        51,
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
                                        63,
                                    ),
                                },
                                trai_expr: 5,
                                for_token: TokenIdx(
                                    69,
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
                                        85,
                                    ),
                                },
                                trai_expr: 7,
                                for_token: TokenIdx(
                                    87,
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
                                        97,
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
                                        109,
                                    ),
                                },
                                trai_expr: 12,
                                for_token: TokenIdx(
                                    115,
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
        ],
        principal_item_path_expr_arena: Arena {
            data: [
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Visualize`,
                            token_idx: TokenIdx(
                                40,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Trait(
                            TraitPath(`core::visual::Visualize`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                42,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`mnist::BinaryImage28`, `Struct`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                52,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`mnist::BinaryImage28`, `Struct`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `IntIndex`,
                            token_idx: TokenIdx(
                                68,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Trait(
                            TraitPath(`core::ops::IntIndex`),
                        ),
                    ),
                },
                MajorItemPathExpr::Subitem {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `ops`,
                            token_idx: TokenIdx(
                                66,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            67,
                        ),
                    ),
                    subexpr: 3,
                },
                MajorItemPathExpr::Subitem {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                64,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            65,
                        ),
                    ),
                    subexpr: 4,
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                70,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`mnist::BinaryImage28`, `Struct`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Visualize`,
                            token_idx: TokenIdx(
                                86,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Trait(
                            TraitPath(`core::visual::Visualize`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                88,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                98,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                        ),
                    ),
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `IntIndex`,
                            token_idx: TokenIdx(
                                114,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Trait(
                            TraitPath(`core::ops::IntIndex`),
                        ),
                    ),
                },
                MajorItemPathExpr::Subitem {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `ops`,
                            token_idx: TokenIdx(
                                112,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            113,
                        ),
                    ),
                    subexpr: 10,
                },
                MajorItemPathExpr::Subitem {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                110,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            111,
                        ),
                    ),
                    subexpr: 11,
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                116,
                            ),
                        },
                    ),
                    major_path: MajorEntityPath::MajorItem(
                        MajorItemPath::Type(
                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                        ),
                    ),
                },
            ],
        },
    },
)