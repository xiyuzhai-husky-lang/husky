Ok(
    EntitySynTreeSheet {
        module_path: `std::logic`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Prop`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LogicAnd`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`core::logic::LogicAnd`, `Structure`),
                                ),
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::LogicAnd`, `Structure`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 1,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LogicOr`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajorItemPath::Type(
                                    TypePath(`core::logic::LogicOr`, `Inductive`),
                                ),
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::logic::LogicOr`, `Inductive`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 1,
                        },
                    ),
                },
            ],
        ),
        impl_block_syn_node_table: [],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 1,
                    use_expr_idx: 3,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `core`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            2..3,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::UniversalPrelude {
                                item_path: PrincipalEntityPath::Module(
                                    `core`,
                                ),
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 1,
                    use_expr_idx: 2,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `logic`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
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
                                item_path: PrincipalEntityPath::Module(
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
                    ast_idx: 1,
                    use_expr_idx: 1,
                    visibility: Scope::Pub,
                    progress: Ok(
                        60,
                    ),
                },
            ],
        ),
        errors: [],
    },
)