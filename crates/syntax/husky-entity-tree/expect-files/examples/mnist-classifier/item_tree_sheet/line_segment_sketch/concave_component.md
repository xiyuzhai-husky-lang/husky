```rust
EntityTreeSheet {
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
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 74,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        syn_node_path: MajorItemSynNodePath::Form(
                            FormSynNodePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 77,
                        ident_token: IdentToken {
                            ident: `find_concave_components`,
                            token_idx: TokenIdx(
                                554,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        61..69,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        FormSynNodePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                            Fn,
                        )`, (0)),
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
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `connected_component`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::connected_component`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `raw_contour`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::raw_contour`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `geom2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::geom2d`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment_sketch`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::fermi`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `digits`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 7,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::major`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `main`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::main`, `Val`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::main`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Class`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 23,
                                use_expr_idx: 18,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAll`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 23,
                                use_expr_idx: 18,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `OneVsAllResult`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 23,
                                use_expr_idx: 18,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `narrow_down`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`malamute::narrow_down`, `Ritchie(
                                            Gn,
                                        )`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`malamute::narrow_down`, `Ritchie(
                                            Gn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 23,
                                use_expr_idx: 18,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `MnistLabel`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 24,
                                use_expr_idx: 20,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryImage28`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 24,
                                use_expr_idx: 20,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BinaryGrid28`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 24,
                                use_expr_idx: 20,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `input`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist::input`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist::input`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 24,
                                use_expr_idx: 20,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist::input`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `concave_component`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `convex_component`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::convex_component`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `convexity`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                        },
                        path: PrincipalEntityPath::Module(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `go_right`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `go_left`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `extend_end`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `extend_start`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_line_segments`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 163,
                                use_expr_idx: 0,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 163,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 165,
                                use_expr_idx: 4,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 168,
                                use_expr_idx: 12,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 167,
                                use_expr_idx: 9,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 166,
                                use_expr_idx: 6,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 166,
                                use_expr_idx: 6,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 166,
                                use_expr_idx: 6,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 166,
                                use_expr_idx: 6,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 166,
                                use_expr_idx: 6,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 166,
                                use_expr_idx: 6,
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
                        ast_idx: 69,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 72,
                        use_expr_idx: 11,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 72,
                        use_expr_idx: 11,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 72,
                        use_expr_idx: 11,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 72,
                        use_expr_idx: 11,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 72,
                        use_expr_idx: 11,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 72,
                        use_expr_idx: 11,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `connected_components`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_connected_component`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ignored_connected_components_row_span_sum_sum`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contours`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_raw_contour`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_line_segment_sketch`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `major_concave_components`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_one`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 10,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `FermiMatchResult`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 19,
                                use_expr_idx: 6,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `fermi_match`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 19,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 20,
                                use_expr_idx: 9,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_raw_contours`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 20,
                                use_expr_idx: 9,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 21,
                                use_expr_idx: 12,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 21,
                                use_expr_idx: 12,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 163,
                                        use_expr_idx: 0,
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
                                ast_idx: 21,
                                use_expr_idx: 12,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 163,
                                        use_expr_idx: 0,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 21,
                                use_expr_idx: 12,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 165,
                                        use_expr_idx: 4,
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
                                ast_idx: 21,
                                use_expr_idx: 12,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponentDistribution`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 22,
                                use_expr_idx: 15,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `EffHoles`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 22,
                                use_expr_idx: 15,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConnectedComponent`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
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
                                ast_idx: 22,
                                use_expr_idx: 15,
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
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_connected_components`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 22,
                                use_expr_idx: 15,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_six`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 11,
                                        use_expr_idx: 2,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_zero`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 12,
                                        use_expr_idx: 5,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_two`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 13,
                                        use_expr_idx: 8,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_three`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 14,
                                        use_expr_idx: 11,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_five`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 15,
                                        use_expr_idx: 14,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_seven`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 16,
                                        use_expr_idx: 17,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_eight`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 17,
                                        use_expr_idx: 20,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_nine`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::Use(
                                    UseSymbol {
                                        original_symbol: EntitySymbol::MajorItem {
                                            major_item_path: MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            ),
                                        },
                                        path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                            ),
                                        ),
                                        visibility: Scope::Pub,
                                        ast_idx: 18,
                                        use_expr_idx: 23,
                                    },
                                ),
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 18,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 73,
                        use_expr_idx: 14,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
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
                        ast_idx: 70,
                        use_expr_idx: 3,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `is_convex`,
                visible_scope: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                        ast_idx: 71,
                        use_expr_idx: 7,
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
                                    path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
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
                                        path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 75,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            49,
                        ),
                    },
                    trai_expr: 17,
                    for_token: TokenIdx(
                        51,
                    ),
                    ty_sketch_expr: Path(
                        18,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    1..2,
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
                                                value: 299,
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
                                                    value: 299,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 76,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            68,
                        ),
                    },
                    ty_expr: 19,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            41..53,
                        ),
                    },
                },
            ),
        ),
    ],
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 69,
                use_expr_idx: 2,
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
            OnceUseRule {
                ast_idx: 70,
                use_expr_idx: 6,
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
                        5..6,
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
            OnceUseRule {
                ast_idx: 71,
                use_expr_idx: 10,
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
                        9..10,
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
            OnceUseRule {
                ast_idx: 72,
                use_expr_idx: 13,
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
                        12..13,
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
            OnceUseRule {
                ast_idx: 73,
                use_expr_idx: 15,
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
                        14..15,
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
            OnceUseRule {
                ast_idx: 69,
                use_expr_idx: 1,
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
                        0..1,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier`,
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 70,
                use_expr_idx: 5,
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
                        4..5,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier`,
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 71,
                use_expr_idx: 9,
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
                        8..9,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier`,
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 72,
                use_expr_idx: 12,
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
                        11..12,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier`,
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 70,
                use_expr_idx: 4,
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
                        3..4,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 71,
                use_expr_idx: 8,
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
                        7..8,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId(
                                    Id {
                                        value: 38,
                                    },
                                ),
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
                ast_idx: 73,
                use_expr_idx: 14,
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
                ast_idx: 69,
                use_expr_idx: 0,
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
                ast_idx: 72,
                use_expr_idx: 11,
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
                ast_idx: 70,
                use_expr_idx: 3,
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
                ast_idx: 71,
                use_expr_idx: 7,
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
```