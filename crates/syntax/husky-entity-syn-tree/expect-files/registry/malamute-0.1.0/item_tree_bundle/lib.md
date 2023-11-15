EntitySynTreeCrateBundle {
    sheets: [
        EntitySynTreeSheet {
            module_path: `malamute`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`malamute::Class`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 11,
                                ident_token: IdentToken {
                                    ident: `Class`,
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                                block: Type {
                                    path: TypePath(
                                        Id {
                                            value: 72,
                                        },
                                    ),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`malamute::Class`, `Enum`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                        ident: `Class`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 13,
                                ident_token: IdentToken {
                                    ident: `OneVsAll`,
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                                block: Type {
                                    path: TypePath(
                                        Id {
                                            value: 73,
                                        },
                                    ),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                3..5,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
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
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 16,
                                ident_token: IdentToken {
                                    ident: `OneVsAllResult`,
                                    token_idx: TokenIdx(
                                        88,
                                    ),
                                },
                                block: Type {
                                    path: TypePath(
                                        Id {
                                            value: 74,
                                        },
                                    ),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                6..9,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
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
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 18,
                                ident_token: IdentToken {
                                    ident: `narrow_down`,
                                    token_idx: TokenIdx(
                                        136,
                                    ),
                                },
                                block: Fugitive {
                                    path: FugitivePath(
                                        Id {
                                            value: 80,
                                        },
                                    ),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
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
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `Class`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAll`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAllResult`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `narrow_down`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                            ),
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
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::Class`, `Enum`),
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
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`malamute::Class`, `Enum`),
                                    ),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 14,
                            impl_regional_token: ImplToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                            trai_expr: 3,
                            for_token: TokenIdx(
                                68,
                            ),
                            ty_sketch_expr: Path(
                                4,
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
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath {
                            path: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                    ),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 17,
                            impl_regional_token: ImplToken {
                                token_idx: TokenIdx(
                                    105,
                                ),
                            },
                            trai_expr: 7,
                            for_token: TokenIdx(
                                124,
                            ),
                            ty_sketch_expr: Path(
                                8,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            9..10,
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
    principal_item_path_expr_arena: Arena {
        data: [
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Unveil`,
                        token_idx: TokenIdx(
                            64,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            62,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        63,
                    ),
                ),
                subexpr: 1,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            60,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        61,
                    ),
                ),
                subexpr: 2,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Class`,
                        token_idx: TokenIdx(
                            69,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Unveil`,
                        token_idx: TokenIdx(
                            120,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            118,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        119,
                    ),
                ),
                subexpr: 5,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            116,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        117,
                    ),
                ),
                subexpr: 6,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `OneVsAll`,
                        token_idx: TokenIdx(
                            125,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            },
        ],
    },
}