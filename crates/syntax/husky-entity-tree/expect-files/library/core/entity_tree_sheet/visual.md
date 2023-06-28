Ok(
    EntityTreeSheet {
        module_path: `core::visual`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Trait(
                                TraitNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::visual::Visualize`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Visualize`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Trait(
                            TraitNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TraitPath(`core::visual::Visualize`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Visualize`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::visual::Html`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Html`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::visual::Html`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Html`,
                    visibility: Scope::Pub,
                },
            ],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Visualize`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Trait(
                            TraitPath(`core::visual::Visualize`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Trait(
                                TraitNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::visual::Visualize`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Visualize`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `Html`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::visual::Html`, `Extern`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::visual::Html`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Html`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_node_table: [],
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)