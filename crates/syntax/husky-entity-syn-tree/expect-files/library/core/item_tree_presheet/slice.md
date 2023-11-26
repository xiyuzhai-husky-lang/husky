EntitySynTreePresheet {
    module_path: `core::slice`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNodeData {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 10,
                        ident_token: IdentToken {
                            ident: `Slice`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::Slice`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `Slice`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNodeData {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 13,
                        ident_token: IdentToken {
                            ident: `CyclicSlice`,
                            token_idx: TokenIdx(
                                54,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `CyclicSlice`,
                visibility: Scope::Pub,
            },
        ],
    },
    use_one_trackers: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 9,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `core::slice`,
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
        ],
    },
    errors: [],
}