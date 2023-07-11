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
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 5,
                                    ident_token: IdentToken {
                                        ident: `OneVsAll`,
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
                                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `OneVsAll`,
                            visibility: Scope::Pub,
                        },
                        EntityNodeEntry {
                            node: EntityNode::ModuleItem(
                                ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 6,
                                    ident_token: IdentToken {
                                        ident: `OneVsAllResult`,
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                },
                            ),
                            node_path: EntityNodePath::ModuleItem(
                                ModuleItemNodePath::Type(
                                    TypeNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                            ident: `OneVsAllResult`,
                            visibility: Scope::Pub,
                        },
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
                                    ast_idx: 8,
                                    ident_token: IdentToken {
                                        ident: `narrow_down`,
                                        token_idx: TokenIdx(
                                            55,
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
                            ident: `OneVsAll`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 5,
                                    ident_token: IdentToken {
                                        ident: `OneVsAll`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `OneVsAllResult`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 6,
                                    ident_token: IdentToken {
                                        ident: `OneVsAllResult`,
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                },
                            },
                        },
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
                                    ast_idx: 8,
                                    ident_token: IdentToken {
                                        ident: `narrow_down`,
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_node_table: [
                    (
                        ImplBlockNodePath::IllFormedImplBlock(
                            IllFormedImplBlockNodePath {
                                path: IllFormedImplBlockPath {
                                    module_path: `malamute`,
                                    disambiguator: 0,
                                },
                            },
                        ),
                        ImplBlockNode::IllFormedImplBlock(
                            IllFormedImplBlockNode {
                                node_path: IllFormedImplBlockNodePath {
                                    path: IllFormedImplBlockPath {
                                        module_path: `malamute`,
                                        disambiguator: 0,
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                                ast_idx: 7,
                                items: None,
                                ill_form: ImplBlockIllForm::MajorPath(
                                    MajorPathExprError::Original(
                                        OriginalMajorPathExprError::UnrecognizedIdent(
                                            IdentToken {
                                                ident: `core`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                        ),
                                    ),
                                ),
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
        ],
        principal_entity_path_expr_arena: Arena {
            data: [],
        },
    },
)