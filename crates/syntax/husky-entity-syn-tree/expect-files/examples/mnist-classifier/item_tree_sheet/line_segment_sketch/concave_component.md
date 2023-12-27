EntitySynTreeSheet {
    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
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
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 75,
                        ident_token: IdentToken {
                            ident: `ConcaveComponent`,
                            token_idx: TokenIdx(
                                35,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `ConcaveComponent`,
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
                                                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 78,
                        ident_token: IdentToken {
                            ident: `find_concave_components`,
                            token_idx: TokenIdx(
                                554,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        62..70,
                                    ),
                                },
                            ),
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
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `find_concave_components`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    module_item_path: MajorItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    module_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `connected_component`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::connected_component`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::connected_component`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `raw_contour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::raw_contour`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::raw_contour`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `geom2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::geom2d`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::geom2d`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment_sketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::fermi`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::fermi`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `digits`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::digits`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::major`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::major`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `main`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::main`, `Val`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::main`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Class`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`malamute::Class`, `Enum`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::Class`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 24,
                                use_expr_idx: 19,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAll`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 24,
                                use_expr_idx: 19,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAllResult`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 24,
                                use_expr_idx: 19,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `narrow_down`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 24,
                                use_expr_idx: 19,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `MnistLabel`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 25,
                                use_expr_idx: 21,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::MnistLabel`, `Enum`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryImage28`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 25,
                                use_expr_idx: 21,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::BinaryImage28`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryGrid28`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 25,
                                use_expr_idx: 21,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `input`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist::input`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist::input`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 25,
                                use_expr_idx: 21,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist::input`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `concave_component`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `convex_component`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::convex_component`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::convex_component`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `convexity`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::convexity`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::line_segment`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `go_right`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `go_left`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `extend_end`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `extend_start`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_line_segments`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 164,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 164,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 166,
                                use_expr_idx: 5,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 169,
                                use_expr_idx: 13,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 168,
                                use_expr_idx: 10,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 167,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 167,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 167,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 167,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 167,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                ast_idx: 167,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 70,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 12,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 12,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 12,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 12,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 12,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 12,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `connected_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_connected_component`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ignored_connected_components_row_span_sum_sum`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contours`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_line_segment_sketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_concave_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 1,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_one`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 11,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `FermiMatchResult`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi_match`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 7,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 21,
                                use_expr_idx: 10,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_raw_contours`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 21,
                                use_expr_idx: 10,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 22,
                                use_expr_idx: 13,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 22,
                                use_expr_idx: 13,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 164,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 22,
                                use_expr_idx: 13,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 164,
                                        use_expr_idx: 1,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 22,
                                use_expr_idx: 13,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 166,
                                        use_expr_idx: 5,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 22,
                                use_expr_idx: 13,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponentDistribution`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 23,
                                use_expr_idx: 16,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `EffHoles`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 23,
                                use_expr_idx: 16,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponent`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 23,
                                use_expr_idx: 16,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_connected_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 23,
                                use_expr_idx: 16,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_six`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 12,
                                        use_expr_idx: 3,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_zero`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 13,
                                        use_expr_idx: 6,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_two`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 14,
                                        use_expr_idx: 9,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_three`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 15,
                                        use_expr_idx: 12,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_five`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 16,
                                        use_expr_idx: 15,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_seven`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 17,
                                        use_expr_idx: 18,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_eight`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 18,
                                        use_expr_idx: 21,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_nine`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            module_item_path: MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Fugitive(
                                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 19,
                                        use_expr_idx: 24,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 74,
                        use_expr_idx: 15,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 71,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_convex`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 72,
                        use_expr_idx: 8,
                    },
                ),
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
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 76,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            49,
                        ),
                    },
                    trai_expr: 18,
                    for_token: TokenIdx(
                        51,
                    ),
                    ty_sketch_expr: Path(
                        19,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    2..3,
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
                                                value: 304,
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
                                                    value: 304,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 77,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            68,
                        ),
                    },
                    ty_expr: 20,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            42..54,
                        ),
                    },
                },
            ),
        ),
    ],
    once_use_rules: UseOneRules(
        [
            UseOneRule {
                ast_idx: 70,
                use_expr_idx: 3,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        2..3,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 71,
                use_expr_idx: 7,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        6..7,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 72,
                use_expr_idx: 11,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        10..11,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 73,
                use_expr_idx: 14,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        13..14,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 74,
                use_expr_idx: 16,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        15..16,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 70,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        1..2,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `mnist_classifier`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 71,
                use_expr_idx: 6,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        5..6,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `mnist_classifier`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 72,
                use_expr_idx: 10,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        9..10,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `mnist_classifier`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 73,
                use_expr_idx: 13,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `geom2d`,
                            token_idx: TokenIdx(
                                26,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        12..13,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `mnist_classifier`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::geom2d`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 71,
                use_expr_idx: 5,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment`,
                            token_idx: TokenIdx(
                                12,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        4..5,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::line_segment`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
            UseOneRule {
                ast_idx: 72,
                use_expr_idx: 9,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `convexity`,
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        8..9,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::convexity`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: `mnist_classifier`,
                is_same_crate: true,
                ast_idx: 74,
                use_expr_idx: 15,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                progress: Ok(
                    45,
                ),
            },
            UseAllRule {
                parent_module_path: `mnist_classifier::line_segment_sketch`,
                is_same_crate: true,
                ast_idx: 70,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                progress: Ok(
                    22,
                ),
            },
            UseAllRule {
                parent_module_path: `mnist_classifier::geom2d`,
                is_same_crate: true,
                ast_idx: 73,
                use_expr_idx: 12,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                progress: Ok(
                    6,
                ),
            },
            UseAllRule {
                parent_module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                is_same_crate: true,
                ast_idx: 71,
                use_expr_idx: 4,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                progress: Ok(
                    7,
                ),
            },
            UseAllRule {
                parent_module_path: `mnist_classifier::line_segment_sketch::convexity`,
                is_same_crate: true,
                ast_idx: 72,
                use_expr_idx: 8,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                progress: Ok(
                    31,
                ),
            },
        ],
    ),
    errors: [],
}