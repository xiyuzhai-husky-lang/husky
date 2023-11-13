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
                                    value: 60,
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
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
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
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
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
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
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
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
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
                                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Fugitive(
                                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
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
                    impl_regional_token: ImplToken {
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
}