Ok(
    EntitySynTreeSheet {
        module_path: `core::slice`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::slice::Slice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Slice`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 32,
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
                                    path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Slice`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `CyclicSliceLeashed`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 33,
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
                                    path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `CyclicSliceLeashed`,
                    visibility: Scope::Pub,
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Slice`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::slice::Slice`, `Extern`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::slice::Slice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Slice`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 32,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `CyclicSliceLeashed`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `CyclicSliceLeashed`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 33,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_syn_node_table: [
            (
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath {
                        path: TypeImplBlockPath {
                            module_path: `core::slice`,
                            ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TypeImplBlock(
                    TypeImplBlockSynNode {
                        syn_node_path: TypeImplBlockSynNodePath {
                            path: TypeImplBlockPath {
                                module_path: `core::slice`,
                                ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 5,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                        ty_expr: 51,
                        items: TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..3,
                            ),
                        },
                    },
                ),
            ),
        ],
        once_use_rules: OnceUseRules(
            [],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [],
    },
)