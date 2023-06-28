Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `malamute`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `narrow_down`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Fugitive(
                                    FugitiveNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `narrow_down`,
                            visibility: Scope::Pub,
                        },
                    ],
                },
                entity_symbol_table: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `narrow_down`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Fugitive(
                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Fugitive(
                                        FugitiveNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `narrow_down`,
                                        token_idx: TokenIdx(
                                            2,
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