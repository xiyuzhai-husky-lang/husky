EntitySynTreePresheet {
    module_path: `mnist_classifier::line_segment_sketch::convex_component`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
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
                                    value: 51,
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
    use_one_trackers: OnceUseRules(
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
                state: OnceUseRuleState::Unresolved,
            },
        ],
    ),
    use_all_trackers: UseAllModuleSymbolsRules(
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
                            ident: `line_segment_sketch`,
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
        ],
    },
    errors: [],
}