Ok(
    EntityTreeSheet {
        module_path: `core::list`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `List`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::list::List`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `List`,
                    visibility: Scope::Pub,
                },
            ],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `List`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::list::List`, `Extern`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::list::List`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `List`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_node_table: [
            (
                ImplBlockNodePath::TypeImplBlock(
                    TypeImplBlockNodePath {
                        path: TypeImplBlockPath {
                            module_path: `core::list`,
                            ty_path: TypePath(`core::list::List`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockNode::TypeImplBlock(
                    TypeImplBlockNode {
                        node_path: TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `core::list`,
                                ty_path: TypePath(`core::list::List`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 5,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                        ty_expr: 0,
                        items: TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                0..4,
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