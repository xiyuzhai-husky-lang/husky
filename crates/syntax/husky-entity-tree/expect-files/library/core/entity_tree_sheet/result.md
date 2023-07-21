Ok(
    EntityTreeSheet {
        module_path: `core::result`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntitySynNode::ModuleItem(
                        ModuleItemSynNode {
                            node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::result::Result`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `Result`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 31,
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
                    node_path: EntitySynNodePath::ModuleItem(
                        ModuleItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::result::Result`, `Enum`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Result`,
                    visibility: Scope::Pub,
                },
            ],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Result`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                        node: ModuleItemSynNode {
                            node_path: ModuleItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::result::Result`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `Result`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 31,
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
                    ident: `Ok`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::TypeVariant {
                                ty_variant_path: TypeVariantPath {
                                    parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                                    ident: `Ok`,
                                },
                            },
                            path: PrincipalEntityPath::TypeVariant(
                                TypeVariantPath {
                                    parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                                    ident: `Ok`,
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Err`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::TypeVariant {
                                ty_variant_path: TypeVariantPath {
                                    parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                                    ident: `Err`,
                                },
                            },
                            path: PrincipalEntityPath::TypeVariant(
                                TypeVariantPath {
                                    parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                                    ident: `Err`,
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            use_expr_idx: 0,
                        },
                    ),
                },
            ],
        ),
        impl_block_node_table: [
            (
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: Path(
                                TypePath(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockSynNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNode {
                        node_path: TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: Path(
                                    TypePath(
                                        Id {
                                            value: 31,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 7,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                23,
                            ),
                        },
                        trai_expr: 49,
                        for_token: TokenIdx(
                            41,
                        ),
                        ty_sketch_expr: Path(
                            50,
                        ),
                        items: Some(
                            TraitForType(
                                TraitForTypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        3..5,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
        ],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 5,
                    use_expr_idx: 1,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `Result`,
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
                                    TypePath(`core::result::Result`, `Enum`),
                                ),
                                node: ModuleItemSynNode {
                                    node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::result::Result`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 6,
                                    ident_token: IdentToken {
                                        ident: `Result`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 31,
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
                    ast_idx: 5,
                    use_expr_idx: 0,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::UseAllTypeVariants {
                        parent_ty_path: TypePath(`core::result::Result`, `Enum`),
                    },
                    parent: Some(
                        MajorEntityPath::ModuleItem(
                            ModuleItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
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