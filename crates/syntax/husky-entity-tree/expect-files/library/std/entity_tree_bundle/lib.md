Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntityNode::Submodule(
                                SubmoduleNode {
                                    node_path: SubmoduleNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `std::prelude`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `std`,
                                    ),
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `prelude`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::Submodule(
                                SubmoduleNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `std::prelude`,
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `prelude`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntityNode::Submodule(
                                SubmoduleNode {
                                    node_path: SubmoduleNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `std::logic`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `std`,
                                    ),
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `logic`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::Submodule(
                                SubmoduleNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `std::logic`,
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `logic`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntityNode::Submodule(
                                SubmoduleNode {
                                    node_path: SubmoduleNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `std::ops`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `std`,
                                    ),
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `ops`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::Submodule(
                                SubmoduleNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `std::ops`,
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `ops`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                        },
                    ],
                },
                entity_symbol_table: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `prelude`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: `std::prelude`,
                                node: SubmoduleNode {
                                    node_path: SubmoduleNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `std::prelude`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `std`,
                                    ),
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `prelude`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `logic`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: `std::logic`,
                                node: SubmoduleNode {
                                    node_path: SubmoduleNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `std::logic`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `std`,
                                    ),
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `logic`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `ops`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: `std::ops`,
                                node: SubmoduleNode {
                                    node_path: SubmoduleNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `std::ops`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `std`,
                                    ),
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `ops`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `std::prelude`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                entity_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `std::logic`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                entity_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_node_table: [],
                once_use_rules: OnceUseRules(
                    [
                        OnceUseRule {
                            ast_idx: 0,
                            use_expr_idx: 2,
                            visibility: Scope::Pub,
                            variant: OnceUseRuleVariant::Parent {
                                parent_name_token: PathNameToken::Ident(
                                    IdentToken {
                                        ident: `core`,
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
                            state: OnceUseRuleState::Erroneous,
                        },
                    ],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedRootIdent(
                            IdentToken {
                                ident: `core`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `std::ops`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Trait(
                                        TraitNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`std::ops::Add`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `std::ops`,
                                    ),
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `Add`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Trait(
                                    TraitNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`std::ops::Add`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `Add`,
                            visibility: Scope::PubUnder(
                                `std::ops`,
                            ),
                        },
                    ],
                },
                entity_symbol_table: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Add`,
                            visibility: Scope::PubUnder(
                                `std::ops`,
                            ),
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`std::ops::Add`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Trait(
                                        TraitNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`std::ops::Add`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `std::ops`,
                                    ),
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `Add`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
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