```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: ModulePath(`mnist`),
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::Submodule(
                            SubmoduleSynNode {
                                syn_node_path: SubmoduleSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::Submodule(
                                            SubmoduleSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: SubmoduleItemPath(`mnist::task),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 16,
                                ident_token: IdentToken {
                                    ident: `task`,
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::Submodule(
                            Room32,
                            SubmoduleSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::Submodule(
                                        SubmoduleSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: SubmoduleItemPath(`mnist::task),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                },
                            ),
                        ),
                        ident: `task`,
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
                                ast_idx: 17,
                                ident_token: IdentToken {
                                    ident: `MnistLabel`,
                                    token_idx: TokenIdx(
                                        6,
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
                                ast_idx: 19,
                                ident_token: IdentToken {
                                    ident: `BinaryImage28`,
                                    token_idx: TokenIdx(
                                        36,
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
                                ast_idx: 23,
                                ident_token: IdentToken {
                                    ident: `BinaryGrid28`,
                                    token_idx: TokenIdx(
                                        77,
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
                                    MajorFormSynNodePath(`mnist::input`, `StaticVar`, (0)),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 27,
                                ident_token: IdentToken {
                                    ident: `input`,
                                    token_idx: TokenIdx(
                                        119,
                                    ),
                                },
                                block: DefnBlock::Form {
                                    path: FormPath(`mnist::input`, `StaticVar`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`mnist::input`, `StaticVar`, (0)),
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
                        ident: `task`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist::task),
                        },
                    },
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
                                FormPath(`mnist::input`, `StaticVar`),
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
                            ast_idx: 20,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                            trai_expr: 0,
                            for_token: TokenIdx(
                                40,
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
                            ast_idx: 21,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    50,
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
                            ast_idx: 22,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    62,
                                ),
                            },
                            trai_expr: 5,
                            for_token: TokenIdx(
                                68,
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
                            ast_idx: 24,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    79,
                                ),
                            },
                            trai_expr: 7,
                            for_token: TokenIdx(
                                81,
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
                            ast_idx: 25,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    91,
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
                            ast_idx: 26,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    103,
                                ),
                            },
                            trai_expr: 12,
                            for_token: TokenIdx(
                                109,
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
        },
        EntityTreeSheet {
            module_path: ModulePath(`mnist::task`),
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
                                                            maybe_ambiguous_item_path: TypePath(`mnist::task::MnistTask`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `MnistTask`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`mnist::task::MnistTask`, `Extern`),
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
                                                        maybe_ambiguous_item_path: TypePath(`mnist::task::MnistTask`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `MnistTask`,
                        visibility: Scope::Pub,
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `MnistTask`,
                        visible_scope: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist::task::MnistTask`, `Extern`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [
                (
                    ImplBlockSynNodePath::TypeImplBlock(
                        TypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TypeImplBlock(
                                        TypeImplBlockSynNodePathData {
                                            path: TypeImplBlockPath(`mnist::task::MnistTask(0)`),
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
                                                path: TypeImplBlockPath(`mnist::task::MnistTask(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 3,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            ty_expr: 14,
                            items: TypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    0..1,
                                ),
                            },
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
                        ident: `Visualize`,
                        token_idx: TokenIdx(
                            39,
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
                            41,
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
                            51,
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
                            67,
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
                            65,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        66,
                    ),
                ),
                subexpr: 3,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            63,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        64,
                    ),
                ),
                subexpr: 4,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `BinaryImage28`,
                        token_idx: TokenIdx(
                            69,
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
                            80,
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
                            82,
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
                            92,
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
                            108,
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
                            106,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        107,
                    ),
                ),
                subexpr: 10,
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
                subexpr: 11,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `BinaryGrid28`,
                        token_idx: TokenIdx(
                            110,
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
                        ident: `MnistTask`,
                        token_idx: TokenIdx(
                            8,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist::task::MnistTask`, `Extern`),
                    ),
                ),
            },
        ],
    },
}
```