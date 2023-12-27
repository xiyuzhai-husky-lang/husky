EntitySynTreeCrateBundle {
    sheets: [
        EntitySynTreeSheet {
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
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 17,
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
                                                1..11,
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
                                                        path: TypePath(`mnist::MnistLabel`, `Enum`),
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
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 23,
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
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                                syn_node_path: MajorItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Fugitive(
                                                    FugitiveSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist::input`, `Val`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 27,
                                ident_token: IdentToken {
                                    ident: `input`,
                                    token_idx: TokenIdx(
                                        115,
                                    ),
                                },
                                block: DefnBlock::Fugitive {
                                    path: FugitivePath(`mnist::input`, `Val`),
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
                                                        path: FugitivePath(`mnist::input`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
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
                        },
                    },
                    EntitySymbolEntry {
                        ident: `BinaryImage28`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist::BinaryImage28`, `Extern`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `BinaryGrid28`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `input`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist::input`, `Val`),
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
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                    35,
                                ),
                            },
                            trai_expr: 1,
                            for_token: TokenIdx(
                                37,
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
                        TypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TypeImplBlock(
                                        TypeImplBlockSynNodePathData {
                                            path: TypeImplBlockPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
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
                                                path: TypeImplBlockPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 342,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 21,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    47,
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
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 22,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                            trai_expr: 6,
                            for_token: TokenIdx(
                                65,
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
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 24,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    76,
                                ),
                            },
                            trai_expr: 8,
                            for_token: TokenIdx(
                                78,
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
                        TypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TypeImplBlock(
                                        TypeImplBlockSynNodePathData {
                                            path: TypeImplBlockPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
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
                                                path: TypeImplBlockPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 345,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 25,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    88,
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
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                                                        module_path: `mnist`,
                                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 26,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    100,
                                ),
                            },
                            trai_expr: 13,
                            for_token: TokenIdx(
                                106,
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
                        ident: `Visualize`,
                        token_idx: TokenIdx(
                            36,
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
                            38,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Extern`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `BinaryImage28`,
                        token_idx: TokenIdx(
                            48,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Extern`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `IntIndex`,
                        token_idx: TokenIdx(
                            64,
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
                            62,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        63,
                    ),
                ),
                subexpr: 4,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            60,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        61,
                    ),
                ),
                subexpr: 5,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `BinaryImage28`,
                        token_idx: TokenIdx(
                            66,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Extern`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Visualize`,
                        token_idx: TokenIdx(
                            77,
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
                            79,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `IntIndex`,
                        token_idx: TokenIdx(
                            105,
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
                            103,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        104,
                    ),
                ),
                subexpr: 11,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            101,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        102,
                    ),
                ),
                subexpr: 12,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `BinaryGrid28`,
                        token_idx: TokenIdx(
                            107,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                    ),
                ),
            },
        ],
    },
}