Ok(
    EntitySynTreeSheet {
        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
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
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `ConvexComponent`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 94,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
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
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
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
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `ConvexComponent`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 94,
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
                                    ast_idx: 159,
                                    ident_token: IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            1,
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
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    ast_idx: 160,
                                    ident_token: IdentToken {
                                        ident: `convex_component`,
                                        token_idx: TokenIdx(
                                            3,
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
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    ast_idx: 161,
                                    ident_token: IdentToken {
                                        ident: `convexity`,
                                        token_idx: TokenIdx(
                                            5,
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
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    ast_idx: 162,
                                    ident_token: IdentToken {
                                        ident: `line_segment`,
                                        token_idx: TokenIdx(
                                            7,
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
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 169,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentStroke`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 91,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 172,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentSketch`,
                                        token_idx: TokenIdx(
                                            171,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 92,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
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
                                    ast_idx: 175,
                                    ident_token: IdentToken {
                                        ident: `go_right`,
                                        token_idx: TokenIdx(
                                            370,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 17,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    26..32,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
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
                                    ast_idx: 176,
                                    ident_token: IdentToken {
                                        ident: `go_left`,
                                        token_idx: TokenIdx(
                                            467,
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
                                                    32..38,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
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
                                    ast_idx: 177,
                                    ident_token: IdentToken {
                                        ident: `extend_end`,
                                        token_idx: TokenIdx(
                                            564,
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
                                                    60..72,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
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
                                    ast_idx: 178,
                                    ident_token: IdentToken {
                                        ident: `extend_start`,
                                        token_idx: TokenIdx(
                                            797,
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
                                                    105..115,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
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
                                    ast_idx: 179,
                                    ident_token: IdentToken {
                                        ident: `find_line_segments`,
                                        token_idx: TokenIdx(
                                            1063,
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
                                                    149..159,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 74,
                                            ident_token: IdentToken {
                                                ident: `ConcaveComponent`,
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 93,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 77,
                                            ident_token: IdentToken {
                                                ident: `find_concave_components`,
                                                token_idx: TokenIdx(
                                                    538,
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
                                                            61..69,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 16,
                                            ident_token: IdentToken {
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 95,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 165,
                                    use_expr_idx: 4,
                                },
                            ),
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 16,
                                            ident_token: IdentToken {
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 95,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 203,
                                            ident_token: IdentToken {
                                                ident: `RawContour`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 82,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 78,
                                            ident_token: IdentToken {
                                                ident: `Point2d`,
                                                token_idx: TokenIdx(
                                                    2,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 85,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 80,
                                            ident_token: IdentToken {
                                                ident: `RelativePoint2d`,
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 86,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 81,
                                            ident_token: IdentToken {
                                                ident: `Vector2d`,
                                                token_idx: TokenIdx(
                                                    157,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 87,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 83,
                                            ident_token: IdentToken {
                                                ident: `ClosedRange`,
                                                token_idx: TokenIdx(
                                                    488,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 88,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 85,
                                            ident_token: IdentToken {
                                                ident: `BoundingBox`,
                                                token_idx: TokenIdx(
                                                    596,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 89,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                                    original_symbol: EntitySymbol::ModuleItem {
                                        module_item_path: ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        ),
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 87,
                                            ident_token: IdentToken {
                                                ident: `RelativeBoundingBox`,
                                                token_idx: TokenIdx(
                                                    732,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 90,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::ModuleItem(
                                        ModuleItemPath::Type(
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
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
                            ty_sketch: Path(
                                TypePath(
                                    Id {
                                        value: 94,
                                    },
                                ),
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
                                ty_sketch: Path(
                                    TypePath(
                                        Id {
                                            value: 94,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 4,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                        trai_expr: 20,
                        for_token: TokenIdx(
                            22,
                        ),
                        ty_sketch_expr: Path(
                            21,
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
        ],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 2,
                    use_expr_idx: 2,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
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
                    ast_idx: 2,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::line_segment_sketch::convex_component`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
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
                                            7,
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
                    ast_idx: 2,
                    use_expr_idx: 0,
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