Ok(
    EntitySynTreeCrateBundle {
        sheets: [
            EntitySynTreeSheet {
                module_path: `malamute`,
                major_entity_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 6,
                                    ident_token: IdentToken {
                                        ident: `OneVsAll`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 104,
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
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 7,
                                    ident_token: IdentToken {
                                        ident: `OneVsAllResult`,
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 105,
                                            },
                                        ),
                                        variants: Some(
                                            TypeVariants {
                                                ast_idx_range: ArenaIdxRange(
                                                    2..5,
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
                            node: EntitySynNode::ModuleItem(
                                ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 9,
                                    ident_token: IdentToken {
                                        ident: `narrow_down`,
                                        token_idx: TokenIdx(
                                            61,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                        body: None,
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::ModuleItem(
                                ModuleItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
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
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 6,
                                    ident_token: IdentToken {
                                        ident: `OneVsAll`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 104,
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
                            ident: `OneVsAllResult`,
                            visibility: Scope::Pub,
                            symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 7,
                                    ident_token: IdentToken {
                                        ident: `OneVsAllResult`,
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 105,
                                            },
                                        ),
                                        variants: Some(
                                            TypeVariants {
                                                ast_idx_range: ArenaIdxRange(
                                                    2..5,
                                                ),
                                            },
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
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Fugitive(
                                        FugitiveSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 9,
                                    ident_token: IdentToken {
                                        ident: `narrow_down`,
                                        token_idx: TokenIdx(
                                            61,
                                        ),
                                    },
                                    block: Fugitive {
                                        path: FugitivePath(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                        body: None,
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_syn_node_table: [
                    (
                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                            TraitForTypeImplBlockSynNodePath {
                                path: TraitForTypeImplBlockPath {
                                    module_path: `malamute`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 104,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                            },
                        ),
                        ImplBlockSynNode::TraitForTypeImplBlock(
                            TraitForTypeImplBlockSynNode {
                                syn_node_path: TraitForTypeImplBlockSynNodePath {
                                    path: TraitForTypeImplBlockPath {
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: Path(
                                            TypePath(
                                                Id {
                                                    value: 104,
                                                },
                                            ),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 8,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                                trai_expr: 2,
                                for_token: TokenIdx(
                                    49,
                                ),
                                ty_sketch_expr: Path(
                                    3,
                                ),
                                items: Some(
                                    TraitForType(
                                        TraitForTypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                5..6,
                                            ),
                                        },
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
            data: [
                ModuleItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Unveil`,
                            token_idx: TokenIdx(
                                45,
                            ),
                        },
                    ),
                    major_entity_path: MajorEntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Unveil`),
                        ),
                    ),
                },
                ModuleItemPathExpr::Subentity {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `ops`,
                            token_idx: TokenIdx(
                                43,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            44,
                        ),
                    ),
                    subexpr: 0,
                },
                ModuleItemPathExpr::Subentity {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `core`,
                            token_idx: TokenIdx(
                                41,
                            ),
                        },
                    ),
                    scope_resolution_token: ScopeResolutionToken(
                        TokenIdx(
                            42,
                        ),
                    ),
                    subexpr: 1,
                },
                ModuleItemPathExpr::Root {
                    name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `OneVsAll`,
                            token_idx: TokenIdx(
                                50,
                            ),
                        },
                    ),
                    major_entity_path: MajorEntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                    ),
                },
            ],
        },
    },
)