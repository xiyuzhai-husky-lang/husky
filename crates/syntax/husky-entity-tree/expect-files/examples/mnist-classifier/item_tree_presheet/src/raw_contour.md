```rust
EntityTreePresheet {
    module_path: ModulePath(`mnist_classifier::raw_contour`),
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 194,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 198,
                        ident_token: IdentToken {
                            ident: `Direction`,
                            token_idx: TokenIdx(
                                413,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                        TypeSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Type(
                                        TypeSynNodePathData {
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 199,
                        ident_token: IdentToken {
                            ident: `get_pixel_pair`,
                            token_idx: TokenIdx(
                                423,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        38..39,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_pixel_pair`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 200,
                        ident_token: IdentToken {
                            ident: `get_pixel_to_the_left`,
                            token_idx: TokenIdx(
                                448,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        39..40,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_pixel_to_the_left`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 201,
                        ident_token: IdentToken {
                            ident: `get_pixel_to_the_right`,
                            token_idx: TokenIdx(
                                469,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        40..41,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_pixel_to_the_right`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 202,
                        ident_token: IdentToken {
                            ident: `get_inward_direction`,
                            token_idx: TokenIdx(
                                494,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        63..66,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_inward_direction`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 203,
                        ident_token: IdentToken {
                            ident: `get_angle_change`,
                            token_idx: TokenIdx(
                                622,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        70..72,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_angle_change`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 204,
                        ident_token: IdentToken {
                            ident: `get_outward_direction`,
                            token_idx: TokenIdx(
                                683,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        110..113,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_outward_direction`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
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
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 205,
                        ident_token: IdentToken {
                            ident: `StreakCache`,
                            token_idx: TokenIdx(
                                879,
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
                                            disambiguated_item_path: DisambiguatedItemPath {
                                                maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::raw_contour`),
                        ),
                        ast_idx: 206,
                        ident_token: IdentToken {
                            ident: `get_concave_middle_point`,
                            token_idx: TokenIdx(
                                891,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        113..117,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `get_concave_middle_point`,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 207,
                        ident_token: IdentToken {
                            ident: `find_raw_contours`,
                            token_idx: TokenIdx(
                                957,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        185..190,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `find_raw_contours`,
                visibility: Scope::Pub,
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 190,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
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
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 191,
                use_expr_idx: 5,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
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
                        4..5,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 192,
                use_expr_idx: 8,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
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
                        7..8,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 193,
                use_expr_idx: 10,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::raw_contour`),
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
                        9..10,
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
                ParentUseExprData {
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
                            child: 0,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
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
                            child: 1,
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
                ParentUseExprData {
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
                            child: 3,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
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
                            child: 4,
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
                ParentUseExprData {
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
                            child: 6,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
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
                            child: 7,
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
                ParentUseExprData {
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
                            child: 9,
                        },
                    ),
                },
            ),
        ],
    },
}
```