Ok(
    EntitySynTreeCrateBundle {
        sheets: [
            EntitySynTreeSheet {
                module_path: `std`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
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
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
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
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
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
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
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
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
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
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
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
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
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
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
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
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
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
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
                module_path: `std::prelude`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                entity_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
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
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
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
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
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
                                        node: ModuleItemSynNode {
                                            syn_node_path: ModuleItemSynNodePath::Type(
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
            EntitySynTreeSheet {
                module_path: `std::ops`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
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
                                    block: Trait {
                                        path: TraitPath(
                                            Id {
                                                value: 52,
                                            },
                                        ),
                                        items: Some(
                                            TraitItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    0..2,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Trait(
                                    TraitSynNodePath {
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
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
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
                                    block: Trait {
                                        path: TraitPath(
                                            Id {
                                                value: 52,
                                            },
                                        ),
                                        items: Some(
                                            TraitItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    0..2,
                                                ),
                                            },
                                        ),
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_syn_node_table: [],
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