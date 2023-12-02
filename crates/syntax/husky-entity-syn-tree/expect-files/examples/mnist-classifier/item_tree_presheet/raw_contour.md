EntitySynTreePresheet {
    module_path: `mnist_classifier::raw_contour`,
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
                                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 195,
                        ident_token: IdentToken {
                            ident: `RawContour`,
                            token_idx: TokenIdx(
                                25,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `RawContour`,
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
                                                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 198,
                        ident_token: IdentToken {
                            ident: `Direction`,
                            token_idx: TokenIdx(
                                402,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        35..39,
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
                                                path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `Direction`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 199,
                        ident_token: IdentToken {
                            ident: `get_pixel_pair`,
                            token_idx: TokenIdx(
                                412,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_pixel_pair`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 200,
                        ident_token: IdentToken {
                            ident: `get_pixel_to_the_left`,
                            token_idx: TokenIdx(
                                437,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_pixel_to_the_left`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 201,
                        ident_token: IdentToken {
                            ident: `get_pixel_to_the_right`,
                            token_idx: TokenIdx(
                                458,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        41..42,
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
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_pixel_to_the_right`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 202,
                        ident_token: IdentToken {
                            ident: `get_inward_direction`,
                            token_idx: TokenIdx(
                                483,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        64..67,
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
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_inward_direction`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 203,
                        ident_token: IdentToken {
                            ident: `get_angle_change`,
                            token_idx: TokenIdx(
                                611,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        71..73,
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
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_angle_change`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 204,
                        ident_token: IdentToken {
                            ident: `get_outward_direction`,
                            token_idx: TokenIdx(
                                672,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        111..114,
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
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_outward_direction`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 205,
                        ident_token: IdentToken {
                            ident: `StreakCache`,
                            token_idx: TokenIdx(
                                868,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                                                path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `StreakCache`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                        ast_idx: 206,
                        ident_token: IdentToken {
                            ident: `get_concave_middle_point`,
                            token_idx: TokenIdx(
                                880,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        114..118,
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
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `get_concave_middle_point`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
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
                                                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 207,
                        ident_token: IdentToken {
                            ident: `find_raw_contours`,
                            token_idx: TokenIdx(
                                946,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        186..191,
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
                                                path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `find_raw_contours`,
                visibility: Scope::Pub,
            },
        ],
    },
    use_one_rules: UseOneRules(
        [
            UseOneRule {
                ast_idx: 191,
                use_expr_idx: 3,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
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
                state: UseOneRuleState::Unresolved,
            },
            UseOneRule {
                ast_idx: 192,
                use_expr_idx: 6,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
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
                state: UseOneRuleState::Unresolved,
            },
            UseOneRule {
                ast_idx: 193,
                use_expr_idx: 9,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                14,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        8..9,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            UseOneRule {
                ast_idx: 194,
                use_expr_idx: 11,
                visibility: Scope::PubUnder(
                    `mnist_classifier::raw_contour`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        10..11,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        6,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `geom2d`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                5,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 1,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                3,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 2,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        12,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `connected_component`,
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                11,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 4,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                9,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 5,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        18,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                17,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 7,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                14,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                15,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 8,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        22,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                21,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 10,
                        },
                    ),
                },
            ),
        ],
    },
    errors: [],
}