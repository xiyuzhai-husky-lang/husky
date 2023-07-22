Ok(
    EntitySynTreeSheet {
        module_path: `core::option`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::option::Option`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Option`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 29,
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
                    ),
                    syn_node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::option::Option`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Option`,
                    visibility: Scope::Pub,
                },
            ],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Option`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::option::Option`, `Enum`),
                        ),
                        node: ModuleItemSynNode {
                            syn_node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::option::Option`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Option`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 29,
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
                },
                EntitySymbolEntry {
                    ident: `Some`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::TypeVariant {
                                ty_variant_path: TypeVariantPath {
                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                    ident: `Some`,
                                },
                            },
                            path: PrincipalEntityPath::TypeVariant(
                                TypeVariantPath {
                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                    ident: `Some`,
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `None`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::TypeVariant {
                                ty_variant_path: TypeVariantPath {
                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                    ident: `None`,
                                },
                            },
                            path: PrincipalEntityPath::TypeVariant(
                                TypeVariantPath {
                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                    ident: `None`,
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
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
                    ast_idx: 2,
                    use_expr_idx: 1,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `Option`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: None,
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::option::Option`, `Enum`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::option::Option`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `Option`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 29,
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
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 2,
                    use_expr_idx: 0,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::UseAllTypeVariants {
                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                    },
                    parent: Some(
                        MajorEntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: None,
                    },
                },
            ],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [],
        ),
        errors: [],
    },
)