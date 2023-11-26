EntitySynTreePresheet {
    module_path: `core::result`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNodeData {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::result::Result`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 8,
                        ident_token: IdentToken {
                            ident: `Result`,
                            token_idx: TokenIdx(
                                12,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::result::Result`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        1..3,
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
                                path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `Result`,
                visibility: Scope::Pub,
            },
        ],
    },
    use_one_trackers: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 6,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `core::result`,
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
                state: OnceUseRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 7,
                use_expr_idx: 4,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Result`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        3..4,
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
                        4,
                    ),
                ),
            },
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
                            child: 1,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        9,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExpr {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Result`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                8,
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
        ],
    },
    errors: [],
}