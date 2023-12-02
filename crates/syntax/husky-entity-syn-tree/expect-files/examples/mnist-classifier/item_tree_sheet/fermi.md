EntitySynTreeSheet {
    module_path: `mnist_classifier::fermi`,
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
                                                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                            ident: `FermiMatchResult`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `FermiMatchResult`,
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
                                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 25,
                        ident_token: IdentToken {
                            ident: `fermi_match`,
                            token_idx: TokenIdx(
                                154,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        18..22,
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
                                                path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `fermi_match`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `FermiMatchResult`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    module_item_path: MajorItemPath::Type(
                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `fermi_match`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    module_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `connected_component`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `raw_contour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `geom2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment_sketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `digits`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `main`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Class`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAll`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAllResult`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `narrow_down`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `MnistLabel`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryImage28`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                TypePath(`mnist::BinaryImage28`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryGrid28`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    module_item_path: MajorItemPath::Type(
                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist::BinaryGrid28`, `Struct`),
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
                                TypePath(`mnist::BinaryGrid28`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `input`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `connected_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_connected_component`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ignored_connected_components_row_span_sum_sum`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contours`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_line_segment_sketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_concave_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_one`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `FermiMatchResult`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi_match`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_raw_contours`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponentDistribution`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `EffHoles`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponent`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_connected_components`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_six`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_zero`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_two`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_three`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_five`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_seven`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_eight`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_nine`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                            `mnist_classifier::fermi`,
                        ),
                        ast_idx: 22,
                        use_expr_idx: 1,
                    },
                ),
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
                                    path: TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 305,
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
                                                    value: 305,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 24,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            25,
                        ),
                    },
                    ty_expr: 24,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            13..16,
                        ),
                    },
                },
            ),
        ),
    ],
    once_use_rules: UseOneRules(
        [
            UseOneRule {
                ast_idx: 22,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
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
                        1..2,
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
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: `mnist_classifier`,
                is_same_crate: true,
                ast_idx: 22,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `mnist_classifier::fermi`,
                ),
                progress: Ok(
                    45,
                ),
            },
        ],
    ),
    errors: [],
}