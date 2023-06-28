Ok(
    EntityTreeSheet {
        module_path: `core::result`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::result::Result`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Result`,
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
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::result::Result`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Result`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    },
                },
            ],
        ),
        impl_block_node_table: [
            (
                ImplBlockNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_path: TypePath(`core::result::Result`, `Enum`),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockNode {
                        node_path: TraitForTypeImplBlockNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `core::result`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 6,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                        trai_expr: 47,
                        for_token: TokenIdx(
                            36,
                        ),
                        ty_expr: 48,
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
        use_expr_rules: UseExprRules(
            [],
        ),
        use_all_rules: UseAllRules(
            [],
        ),
        errors: [],
    },
)