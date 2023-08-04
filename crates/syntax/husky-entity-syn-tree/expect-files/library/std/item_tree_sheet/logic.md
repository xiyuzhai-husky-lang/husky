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
                                module_item_path: MajarItemPath::Type(
                                    TypePath(`core::logic::Prop`, `Extern`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
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
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 74,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajarItemPath::Type(
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
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajarItemPath::Type(
                                    TypePath(`core::logic::LogicAnd`, `Structure`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
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
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 75,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajarItemPath::Type(
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
                            original_symbol: EntitySymbol::MajorItem {
                                module_item_path: MajarItemPath::Type(
                                    TypePath(`core::logic::LogicOr`, `Inductive`),
                                ),
                                node: MajorItemSynNode {
                                    syn_node_path: MajorItemSynNodePath::Type(
                                        TypeSynNodePath {
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
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 76,
                                            },
                                        ),
                                        variants: Some(
                                            TypeVariants {
                                                ast_idx_range: ArenaIdxRange(
                                                    0..2,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::MajorItem(
                                MajarItemPath::Type(
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
        impl_block_syn_node_table: [],
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
                                item_path: PrincipalEntityPath::Module(
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