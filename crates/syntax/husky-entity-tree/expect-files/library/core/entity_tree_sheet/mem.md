Ok(
    EntityTreeSheet {
        module_path: `core::mem`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Ref`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Ref`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::Ref`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Ref`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::RefMut`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `RefMut`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::RefMut`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `RefMut`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: EntityNode::ModuleItem(
                        ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Leash`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Leash`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    node_path: EntityNodePath::ModuleItem(
                        ModuleItemNodePath::Type(
                            TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::mem::Leash`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Leash`,
                    visibility: Scope::Pub,
                },
            ],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `Ref`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::mem::Ref`, `Extern`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Ref`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Ref`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 11,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `RefMut`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::mem::RefMut`, `Extern`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::RefMut`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `RefMut`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 12,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `Leash`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem {
                        module_item_path: ModuleItemPath::Type(
                            TypePath(`core::mem::Leash`, `Extern`),
                        ),
                        node: ModuleItemNode {
                            node_path: ModuleItemNodePath::Type(
                                TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Leash`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `Leash`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 13,
                                    },
                                ),
                                variants: None,
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
                            module_path: `core::mem`,
                            trai_path: TraitPath(`core::marker::Copy`),
                            ty_sketch: Path(
                                TypePath(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            ),
                            disambiguator: 0,
                        },
                    },
                ),
                ImplBlockNode::TraitForTypeImplBlock(
                    TraitForTypeImplBlockNode {
                        node_path: TraitForTypeImplBlockNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `core::mem`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_sketch: Path(
                                    TypePath(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                                disambiguator: 0,
                            },
                        },
                        ast_idx: 3,
                        impl_token: ImplToken {
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                        trai_expr: 2,
                        for_token: TokenIdx(
                            35,
                        ),
                        ty_sketch_expr: Path(
                            3,
                        ),
                        items: None,
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
)