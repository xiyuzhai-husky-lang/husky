Ok(
    EntitySynTreeSheet {
        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `ConvexComponent`,
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 58,
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
                                    path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `ConvexComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `ConvexComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `ConvexComponent`,
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 58,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `concave_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 160,
                                    ident_token: IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `convex_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch::convex_component`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::convex_component`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 161,
                                    ident_token: IdentToken {
                                        ident: `convex_component`,
                                        token_idx: TokenIdx(
                                            4,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `convexity`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::convexity`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 162,
                                    ident_token: IdentToken {
                                        ident: `convexity`,
                                        token_idx: TokenIdx(
                                            6,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch::line_segment`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch::line_segment`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 163,
                                    ident_token: IdentToken {
                                        ident: `line_segment`,
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 170,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentStroke`,
                                        token_idx: TokenIdx(
                                            42,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 55,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 173,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentSketch`,
                                        token_idx: TokenIdx(
                                            171,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 56,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `go_right`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 176,
                                    ident_token: IdentToken {
                                        ident: `go_right`,
                                        token_idx: TokenIdx(
                                            370,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    27..33,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `go_left`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 177,
                                    ident_token: IdentToken {
                                        ident: `go_left`,
                                        token_idx: TokenIdx(
                                            467,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    33..39,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `extend_end`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 178,
                                    ident_token: IdentToken {
                                        ident: `extend_end`,
                                        token_idx: TokenIdx(
                                            564,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 20,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    61..73,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `extend_start`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 179,
                                    ident_token: IdentToken {
                                        ident: `extend_start`,
                                        token_idx: TokenIdx(
                                            797,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 21,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    106..116,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_line_segments`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::line_segment_sketch`,
                                    ),
                                    ast_idx: 180,
                                    ident_token: IdentToken {
                                        ident: `find_line_segments`,
                                        token_idx: TokenIdx(
                                            1063,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    150..160,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 75,
                                            ident_token: IdentToken {
                                                ident: `ConcaveComponent`,
                                                token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 57,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 78,
                                            ident_token: IdentToken {
                                                ident: `find_concave_components`,
                                                token_idx: TokenIdx(
                                                    554,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            62..70,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 164,
                                    use_expr_idx: 1,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 17,
                                            ident_token: IdentToken {
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    9,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 59,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 17,
                                            ident_token: IdentToken {
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    9,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 59,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 204,
                                            ident_token: IdentToken {
                                                ident: `RawContour`,
                                                token_idx: TokenIdx(
                                                    25,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 80,
                                            ident_token: IdentToken {
                                                ident: `Point2d`,
                                                token_idx: TokenIdx(
                                                    12,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 49,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 82,
                                            ident_token: IdentToken {
                                                ident: `RelativePoint2d`,
                                                token_idx: TokenIdx(
                                                    154,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 50,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 83,
                                            ident_token: IdentToken {
                                                ident: `Vector2d`,
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 85,
                                            ident_token: IdentToken {
                                                ident: `ClosedRange`,
                                                token_idx: TokenIdx(
                                                    496,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 52,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 87,
                                            ident_token: IdentToken {
                                                ident: `BoundingBox`,
                                                token_idx: TokenIdx(
                                                    604,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 89,
                                            ident_token: IdentToken {
                                                ident: `RelativeBoundingBox`,
                                                token_idx: TokenIdx(
                                                    740,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 54,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 3,
                            use_expr_idx: 1,
                        },
                    ),
                },
            ],
        ),
        impl_block_syn_node_table: [
            (
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        syn_node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 5,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                22,
                            ),
                        },
                        trai_expr: 21,
                        for_token: TokenIdx(
                            24,
                        ),
                        ty_sketch_expr: Path(
                            22,
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
        ],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 3,
                    use_expr_idx: 3,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
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
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::CrateRoot {
                                root_module_path: `mnist_classifier`,
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 3,
                    use_expr_idx: 2,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
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
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::line_segment_sketch`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 14,
                                    ident_token: IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
            ],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [
                UseAllModuleSymbolsRule {
                    parent_module_path: `mnist_classifier::line_segment_sketch`,
                    is_same_crate: true,
                    ast_idx: 3,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    progress: Ok(
                        22,
                    ),
                },
            ],
        ),
        errors: [],
    },
)