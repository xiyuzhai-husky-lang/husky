EntitySynTreePresheet {
    module_path: `mnist`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            EntityNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::MnistLabel`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 17,
                        ident_token: IdentToken {
                            ident: `MnistLabel`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                        block: Type {
                            path: TypePath(
                                Id {
                                    value: 64,
                                },
                            ),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        1..11,
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
                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `MnistLabel`,
                visibility: Scope::Pub,
            },
            EntityNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 19,
                        ident_token: IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                33,
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
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `BinaryImage28`,
                visibility: Scope::Pub,
            },
            EntityNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 23,
                        ident_token: IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                79,
                            ),
                        },
                        block: Type {
                            path: TypePath(
                                Id {
                                    value: 66,
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
                                path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `BinaryGrid28`,
                visibility: Scope::Pub,
            },
            EntityNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist::input`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `input`,
                            token_idx: TokenIdx(
                                125,
                            ),
                        },
                        block: Fugitive {
                            path: FugitivePath(
                                Id {
                                    value: 79,
                                },
                            ),
                            body: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist::input`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ident: `input`,
                visibility: Scope::Pub,
            },
        ],
    },
    use_one_trackers: OnceUseRules(
        [],
    ),
    use_all_trackers: UseAllModuleSymbolsRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [],
    },
    errors: [],
}