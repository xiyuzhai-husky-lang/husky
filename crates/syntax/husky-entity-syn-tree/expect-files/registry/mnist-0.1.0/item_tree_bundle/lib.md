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
                                    ast_idx: 17,
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
                                                    1..11,
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
                                    ast_idx: 19,
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
                                    ast_idx: 23,
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
                                    ast_idx: 27,
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
                                    ast_idx: 17,
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
                                                    1..11,
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
                                    ast_idx: 19,
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
                                    ast_idx: 23,
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
                                    ast_idx: 27,
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
                                ast_idx: 20,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        40,
                                    ),
                                },
                                trai_expr: 1,
                                for_token: TokenIdx(
                                    42,
                                ),
                                ty_sketch_expr: Path(
                                    2,
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
                                ast_idx: 21,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        52,
                                    ),
                                },
                                ty_expr: 3,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        12..13,
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
                                ast_idx: 22,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        64,
                                    ),
                                },
                                trai_expr: 6,
                                for_token: TokenIdx(
                                    70,
                                ),
                                ty_sketch_expr: Path(
                                    7,
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
                                ast_idx: 24,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        86,
                                    ),
                                },
                                trai_expr: 8,
                                for_token: TokenIdx(
                                    88,
                                ),
                                ty_sketch_expr: Path(
                                    9,
                                ),
                                items: Some(
                                    TraitForType(
                                        TraitForTypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                14..15,
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
                                ast_idx: 25,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        98,
                                    ),
                                },
                                ty_expr: 10,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        15..16,
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
                                ast_idx: 26,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        110,
                                    ),
                                },
                                trai_expr: 13,
                                for_token: TokenIdx(
                                    116,
                                ),
                                ty_sketch_expr: Path(
                                    14,
                                ),
                                items: Some(
                                    TraitForType(
                                        TraitForTypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                16..17,
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
                                41,
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
                                43,
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
                                53,
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
                                69,
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
                                67,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            68,
                        ),
                    ),
                    subexpr: 4,
                },
                MajorItemPathExpr::Subitem {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                65,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            66,
                        ),
                    ),
                    subexpr: 5,
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                71,
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
                                87,
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
                                89,
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
                                99,
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
                                115,
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
                                113,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            114,
                        ),
                    ),
                    subexpr: 11,
                },
                MajorItemPathExpr::Subitem {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                111,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            112,
                        ),
                    ),
                    subexpr: 12,
                },
                MajorItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                117,
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