Ok(
    EntitySynTreeSheet {
        module_path: `mnist_classifier::raw_contour`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
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
                                        value: 46,
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
                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `RawContour`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 206,
                            ident_token: IdentToken {
                                ident: `Direction`,
                                token_idx: TokenIdx(
                                    401,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            34..38,
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
                                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Direction`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 207,
                            ident_token: IdentToken {
                                ident: `get_pixel_pair`,
                                token_idx: TokenIdx(
                                    411,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 9,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            38..39,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_pixel_pair`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 208,
                            ident_token: IdentToken {
                                ident: `get_pixel_to_the_left`,
                                token_idx: TokenIdx(
                                    436,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 10,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            39..40,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_pixel_to_the_left`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 209,
                            ident_token: IdentToken {
                                ident: `get_pixel_to_the_right`,
                                token_idx: TokenIdx(
                                    457,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            40..41,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_pixel_to_the_right`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 210,
                            ident_token: IdentToken {
                                ident: `get_inward_direction`,
                                token_idx: TokenIdx(
                                    482,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            66..69,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_inward_direction`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 211,
                            ident_token: IdentToken {
                                ident: `get_angle_change`,
                                token_idx: TokenIdx(
                                    613,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            75..77,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_angle_change`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 212,
                            ident_token: IdentToken {
                                ident: `get_outward_direction`,
                                token_idx: TokenIdx(
                                    674,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 14,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            119..122,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_outward_direction`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 213,
                            ident_token: IdentToken {
                                ident: `StreakCache`,
                                token_idx: TokenIdx(
                                    876,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 48,
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
                                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `StreakCache`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 214,
                            ident_token: IdentToken {
                                ident: `get_concave_middle_point`,
                                token_idx: TokenIdx(
                                    888,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 15,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            122..126,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `get_concave_middle_point`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 215,
                            ident_token: IdentToken {
                                ident: `find_raw_contours`,
                                token_idx: TokenIdx(
                                    954,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 16,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            194..199,
                                        ),
                                    },
                                ),
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `find_raw_contours`,
                    visibility: Scope::Pub,
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
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
                                        value: 46,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `Direction`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 206,
                            ident_token: IdentToken {
                                ident: `Direction`,
                                token_idx: TokenIdx(
                                    401,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                                variants: Some(
                                    TypeVariants {
                                        ast_idx_range: ArenaIdxRange(
                                            34..38,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_pixel_pair`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 207,
                            ident_token: IdentToken {
                                ident: `get_pixel_pair`,
                                token_idx: TokenIdx(
                                    411,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 9,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            38..39,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_pixel_to_the_left`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 208,
                            ident_token: IdentToken {
                                ident: `get_pixel_to_the_left`,
                                token_idx: TokenIdx(
                                    436,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 10,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            39..40,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_pixel_to_the_right`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 209,
                            ident_token: IdentToken {
                                ident: `get_pixel_to_the_right`,
                                token_idx: TokenIdx(
                                    457,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            40..41,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_inward_direction`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 210,
                            ident_token: IdentToken {
                                ident: `get_inward_direction`,
                                token_idx: TokenIdx(
                                    482,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            66..69,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_angle_change`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 211,
                            ident_token: IdentToken {
                                ident: `get_angle_change`,
                                token_idx: TokenIdx(
                                    613,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            75..77,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_outward_direction`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 212,
                            ident_token: IdentToken {
                                ident: `get_outward_direction`,
                                token_idx: TokenIdx(
                                    674,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 14,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            119..122,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `StreakCache`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 213,
                            ident_token: IdentToken {
                                ident: `StreakCache`,
                                token_idx: TokenIdx(
                                    876,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `get_concave_middle_point`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 214,
                            ident_token: IdentToken {
                                ident: `get_concave_middle_point`,
                                token_idx: TokenIdx(
                                    888,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 15,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            122..126,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `find_raw_contours`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Fugitive(
                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 215,
                            ident_token: IdentToken {
                                ident: `find_raw_contours`,
                                token_idx: TokenIdx(
                                    954,
                                ),
                            },
                            block: Fugitive {
                                path: FugitivePath(
                                    Id {
                                        value: 16,
                                    },
                                ),
                                body: Some(
                                    FugitiveBody {
                                        ast_idx_range: ArenaIdxRange(
                                            194..199,
                                        ),
                                    },
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::connected_component`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `connected_component`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::connected_component`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::raw_contour`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::raw_contour`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 12,
                                    ident_token: IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::raw_contour`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::geom2d`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::geom2d`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 13,
                                    ident_token: IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::geom2d`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
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
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::fermi`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::fermi`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 15,
                                    ident_token: IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::fermi`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `digits`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::digits`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 16,
                                    ident_token: IdentToken {
                                        ident: `digits`,
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::digits`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::major`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::major`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 17,
                                    ident_token: IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `mnist_classifier::major`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `OneVsAll`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 6,
                                            ident_token: IdentToken {
                                                ident: `OneVsAll`,
                                                token_idx: TokenIdx(
                                                    2,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                                variants: Some(
                                                    TypeVariants {
                                                        ast_idx_range: ArenaIdxRange(
                                                            0..2,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 18,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `OneVsAllResult`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 7,
                                            ident_token: IdentToken {
                                                ident: `OneVsAllResult`,
                                                token_idx: TokenIdx(
                                                    17,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                                variants: Some(
                                                    TypeVariants {
                                                        ast_idx_range: ArenaIdxRange(
                                                            2..5,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 18,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `narrow_down`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`malamute::narrow_down`, `Gn`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 9,
                                            ident_token: IdentToken {
                                                ident: `narrow_down`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 77,
                                                    },
                                                ),
                                                body: None,
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`malamute::narrow_down`, `Gn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 24,
                                    use_expr_idx: 18,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `MnistLabel`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
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
                                                        value: 63,
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
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 25,
                                    use_expr_idx: 20,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BinaryImage28`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
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
                                                        value: 64,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                    use_expr_idx: 20,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BinaryGrid28`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
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
                                                        value: 65,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                    use_expr_idx: 20,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `input`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
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
                                                        value: 78,
                                                    },
                                                ),
                                                body: None,
                                            },
                                        },
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
                                    use_expr_idx: 20,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist::input`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Point2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 79,
                                    ident_token: IdentToken {
                                        ident: `Point2d`,
                                        token_idx: TokenIdx(
                                            11,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativePoint2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 81,
                                    ident_token: IdentToken {
                                        ident: `RelativePoint2d`,
                                        token_idx: TokenIdx(
                                            153,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vector2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 82,
                                    ident_token: IdentToken {
                                        ident: `Vector2d`,
                                        token_idx: TokenIdx(
                                            166,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ClosedRange`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 84,
                                    ident_token: IdentToken {
                                        ident: `ClosedRange`,
                                        token_idx: TokenIdx(
                                            495,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BoundingBox`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 86,
                                    ident_token: IdentToken {
                                        ident: `BoundingBox`,
                                        token_idx: TokenIdx(
                                            603,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RelativeBoundingBox`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 88,
                                    ident_token: IdentToken {
                                        ident: `RelativeBoundingBox`,
                                        token_idx: TokenIdx(
                                            739,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 199,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 120,
                                    ident_token: IdentToken {
                                        ident: `ConnectedComponentDistribution`,
                                        token_idx: TokenIdx(
                                            12,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 43,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 121,
                                    ident_token: IdentToken {
                                        ident: `EffHoles`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 123,
                                    ident_token: IdentToken {
                                        ident: `ConnectedComponent`,
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 45,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 127,
                                    ident_token: IdentToken {
                                        ident: `find_connected_components`,
                                        token_idx: TokenIdx(
                                            656,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                        body: Some(
                                            FugitiveBody {
                                                ast_idx_range: ArenaIdxRange(
                                                    114..118,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 200,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                    ast_idx: 172,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentSketch`,
                                        token_idx: TokenIdx(
                                            172,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                            ast_idx: 77,
                                            ident_token: IdentToken {
                                                ident: `find_concave_components`,
                                                token_idx: TokenIdx(
                                                    553,
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
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 19,
                                            ident_token: IdentToken {
                                                ident: `connected_components`,
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 70,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_connected_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 20,
                                            ident_token: IdentToken {
                                                ident: `major_connected_component`,
                                                token_idx: TokenIdx(
                                                    18,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 71,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            6..10,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 21,
                                            ident_token: IdentToken {
                                                ident: `ignored_connected_components_row_span_sum_sum`,
                                                token_idx: TokenIdx(
                                                    69,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 72,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            11..14,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contours`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contours`,
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 73,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            14..15,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 23,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contour`,
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 74,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            15..16,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_line_segment_sketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `major_line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 75,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            16..17,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 25,
                                            ident_token: IdentToken {
                                                ident: `major_concave_components`,
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 76,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            17..18,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
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
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_one`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 63,
                                                    ident_token: IdentToken {
                                                        ident: `is_one`,
                                                        token_idx: TokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    block: Fugitive {
                                                        path: FugitivePath(
                                                            Id {
                                                                value: 29,
                                                            },
                                                        ),
                                                        body: Some(
                                                            FugitiveBody {
                                                                ast_idx_range: ArenaIdxRange(
                                                                    48..51,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 10,
                                            use_expr_idx: 0,
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
                                    use_expr_idx: 3,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `FermiMatchResult`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `FermiMatchResult`,
                                                token_idx: TokenIdx(
                                                    6,
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
                                    use_expr_idx: 6,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi_match`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `fermi_match`,
                                                token_idx: TokenIdx(
                                                    153,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 24,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            17..21,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 20,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 9,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_raw_contours`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 215,
                                            ident_token: IdentToken {
                                                ident: `find_raw_contours`,
                                                token_idx: TokenIdx(
                                                    954,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 16,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            194..199,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 9,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
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
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
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
                                            ast_idx: 172,
                                            ident_token: IdentToken {
                                                ident: `LineSegmentSketch`,
                                                token_idx: TokenIdx(
                                                    172,
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
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
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
                                                    ast_idx: 77,
                                                    ident_token: IdentToken {
                                                        ident: `find_concave_components`,
                                                        token_idx: TokenIdx(
                                                            553,
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
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 120,
                                            ident_token: IdentToken {
                                                ident: `ConnectedComponentDistribution`,
                                                token_idx: TokenIdx(
                                                    12,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 43,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                    use_expr_idx: 15,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 121,
                                            ident_token: IdentToken {
                                                ident: `EffHoles`,
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 44,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                    use_expr_idx: 15,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 123,
                                            ident_token: IdentToken {
                                                ident: `ConnectedComponent`,
                                                token_idx: TokenIdx(
                                                    71,
                                                ),
                                            },
                                            block: Type {
                                                path: TypePath(
                                                    Id {
                                                        value: 45,
                                                    },
                                                ),
                                                variants: None,
                                            },
                                        },
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
                                    use_expr_idx: 15,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::MajorItem {
                                        module_item_path: MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                        ),
                                        node: MajorItemSynNode {
                                            syn_node_path: MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 127,
                                            ident_token: IdentToken {
                                                ident: `find_connected_components`,
                                                token_idx: TokenIdx(
                                                    656,
                                                ),
                                            },
                                            block: Fugitive {
                                                path: FugitivePath(
                                                    Id {
                                                        value: 8,
                                                    },
                                                ),
                                                body: Some(
                                                    FugitiveBody {
                                                        ast_idx_range: ArenaIdxRange(
                                                            114..118,
                                                        ),
                                                    },
                                                ),
                                            },
                                        },
                                    },
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 15,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 202,
                            use_expr_idx: 9,
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
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        syn_node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 204,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                38,
                            ),
                        },
                        trai_expr: 3,
                        for_token: TokenIdx(
                            40,
                        ),
                        ty_sketch_expr: Path(
                            4,
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
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TypeImplBlock(
                    TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 205,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                60,
                            ),
                        },
                        ty_expr: 5,
                        items: TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                29..34,
                            ),
                        },
                    },
                ),
            ),
        ],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 199,
                    use_expr_idx: 2,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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
                    ast_idx: 200,
                    use_expr_idx: 5,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
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
                    ast_idx: 201,
                    use_expr_idx: 8,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
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
                    ast_idx: 202,
                    use_expr_idx: 10,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
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
                    ast_idx: 199,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `geom2d`,
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
                                    `mnist_classifier::geom2d`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::geom2d`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 13,
                                    ident_token: IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 200,
                    use_expr_idx: 4,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `connected_component`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            3..4,
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
                                    `mnist_classifier::connected_component`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `connected_component`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 201,
                    use_expr_idx: 7,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
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
                    parent_module_path: `mnist_classifier`,
                    is_same_crate: true,
                    ast_idx: 202,
                    use_expr_idx: 9,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: Ok(
                        35,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `mnist_classifier::geom2d`,
                    is_same_crate: true,
                    ast_idx: 199,
                    use_expr_idx: 0,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: Ok(
                        6,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `mnist_classifier::connected_component`,
                    is_same_crate: true,
                    ast_idx: 200,
                    use_expr_idx: 3,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    progress: Ok(
                        43,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `mnist_classifier::line_segment_sketch`,
                    is_same_crate: true,
                    ast_idx: 201,
                    use_expr_idx: 6,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::raw_contour`,
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