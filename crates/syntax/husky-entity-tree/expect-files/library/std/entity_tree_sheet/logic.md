Ok(
    EntityTreeSheet {
        module_path: `std::logic`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Prop`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::logic::Prop`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `Prop`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LogicAnd`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::logic::LogicAnd`, `Structure`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `LogicAnd`,
                                        token_idx: TokenIdx(
                                            6,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::logic::LogicAnd`, `Structure`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LogicOr`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::logic::LogicOr`, `Inductive`),
                                ),
                                node: ModuleItemNode {
                                    node_path: ModuleItemNodePath::Type(
                                        TypeNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `LogicOr`,
                                        token_idx: TokenIdx(
                                            28,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::logic::LogicOr`, `Inductive`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
            ],
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
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::UniversalPrelude {
                                entity_path: PrincipalEntityPath::Module(
                                    `core`,
                                ),
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 0,
                    use_expr_idx: 1,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `logic`,
                                token_idx: TokenIdx(
                                    4,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::PackageDependency {
                                entity_path: PrincipalEntityPath::Module(
                                    `core::logic`,
                                ),
                            },
                        ),
                    },
                },
            ],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::logic`,
                    is_same_crate: false,
                    ast_idx: 0,
                    use_expr_idx: 0,
                    visibility: Scope::Pub,
                    progress: Ok(
                        3,
                    ),
                },
            ],
        ),
        errors: [],
    },
)