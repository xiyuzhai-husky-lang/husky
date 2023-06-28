Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 10,
                                    ident_token: IdentToken {
                                        ident: `MnistLabel`,
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
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `BinaryImage28`,
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Type(
                                    TypeNodePath {
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
                    ],
                },
                entity_symbol_table: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 10,
                                    ident_token: IdentToken {
                                        ident: `MnistLabel`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `BinaryImage28`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `BinaryImage28`,
                                        token_idx: TokenIdx(
                                            25,
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
        ],
        principal_entity_path_expr_arena: Arena {
            data: [],
        },
    },
)