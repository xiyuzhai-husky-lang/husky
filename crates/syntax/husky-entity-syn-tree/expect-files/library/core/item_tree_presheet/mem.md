EntitySynTreePresheet {
    module_path: `core::mem`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::Ref`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `Ref`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::mem::Ref`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::Ref`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `Ref`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::RefMut`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 3,
                        ident_token: IdentToken {
                            ident: `RefMut`,
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::mem::RefMut`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::RefMut`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `RefMut`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::Leash`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 4,
                        ident_token: IdentToken {
                            ident: `Leash`,
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::Leash`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `Leash`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::At`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 6,
                        ident_token: IdentToken {
                            ident: `At`,
                            token_idx: TokenIdx(
                                46,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::mem::At`, `Extern`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::mem::At`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `At`,
                visibility: Scope::Pub,
            },
        ],
    },
    use_one_trackers: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 1,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `core::mem`,
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