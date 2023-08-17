Ok(
    EntitySynTreeSheet {
        module_path: `core::basic`,
        major_item_node_table: MajorEntityNodeTable {
            entries: [
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::bool`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `bool`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::bool`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `bool`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::never`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `never`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::never`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `never`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::unit`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `unit`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::unit`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `unit`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::Trait`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `Trait`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 5,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::Trait`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Trait`,
                    visibility: Scope::Pub,
                },
                EntityNodeEntry {
                    node: ItemSynNode::MajorItem(
                        MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::Module`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Module`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    ),
                    syn_node_path: ItemSynNodePath::MajorItem(
                        MajorItemSynNodePath::Type(
                            TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`core::basic::Module`, `Extern`),
                                    disambiguator: 0,
                                },
                            },
                        ),
                    ),
                    ident: `Module`,
                    visibility: Scope::Pub,
                },
            ],
        },
        item_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `bool`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`core::basic::bool`, `Extern`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::bool`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `bool`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `never`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`core::basic::never`, `Extern`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::never`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `never`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `unit`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`core::basic::unit`, `Extern`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::unit`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `unit`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `Trait`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`core::basic::Trait`, `Extern`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::Trait`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `Trait`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 5,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `Module`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::MajorItem {
                        module_item_path: MajorItemPath::Type(
                            TypePath(`core::basic::Module`, `Extern`),
                        ),
                        node: MajorItemSynNode {
                            syn_node_path: MajorItemSynNodePath::Type(
                                TypeSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::basic::Module`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `Module`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                            block: Type {
                                path: TypePath(
                                    Id {
                                        value: 6,
                                    },
                                ),
                                variants: None,
                            },
                        },
                    },
                },
                EntitySymbolEntry {
                    ident: `array`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::array`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::array`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `array`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::array`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `basic`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::basic`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::basic`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `basic`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::basic`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `clone`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::clone`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::clone`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `clone`,
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::clone`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `cmp`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::cmp`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::cmp`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `cmp`,
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::cmp`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `default`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::default`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::default`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `default`,
                                        token_idx: TokenIdx(
                                            14,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::default`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fmt`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::fmt`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::fmt`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 5,
                                    ident_token: IdentToken {
                                        ident: `fmt`,
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::fmt`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `logic`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::logic`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::logic`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 6,
                                    ident_token: IdentToken {
                                        ident: `logic`,
                                        token_idx: TokenIdx(
                                            20,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::logic`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `marker`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::marker`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::marker`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 7,
                                    ident_token: IdentToken {
                                        ident: `marker`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::marker`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `mem`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::mem`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::mem`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 8,
                                    ident_token: IdentToken {
                                        ident: `mem`,
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::mem`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `num`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::num`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::num`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 9,
                                    ident_token: IdentToken {
                                        ident: `num`,
                                        token_idx: TokenIdx(
                                            29,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::num`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ops`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::ops`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::ops`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 10,
                                    ident_token: IdentToken {
                                        ident: `ops`,
                                        token_idx: TokenIdx(
                                            32,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::ops`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `option`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::option`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::option`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `option`,
                                        token_idx: TokenIdx(
                                            35,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::option`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `prelude`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::prelude`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::prelude`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 12,
                                    ident_token: IdentToken {
                                        ident: `prelude`,
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::prelude`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `raw_bits`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::raw_bits`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::raw_bits`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 13,
                                    ident_token: IdentToken {
                                        ident: `raw_bits`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::raw_bits`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `result`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::result`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::result`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 14,
                                    ident_token: IdentToken {
                                        ident: `result`,
                                        token_idx: TokenIdx(
                                            44,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::result`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `vec`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::vec`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::vec`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 15,
                                    ident_token: IdentToken {
                                        ident: `vec`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::vec`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `slice`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::slice`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::slice`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 16,
                                    ident_token: IdentToken {
                                        ident: `slice`,
                                        token_idx: TokenIdx(
                                            50,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::slice`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `str`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::str`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::str`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 17,
                                    ident_token: IdentToken {
                                        ident: `str`,
                                        token_idx: TokenIdx(
                                            53,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::str`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `visual`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `core::visual`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `core::visual`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 18,
                                    ident_token: IdentToken {
                                        ident: `visual`,
                                        token_idx: TokenIdx(
                                            56,
                                        ),
                                    },
                                },
                            },
                            path: PrincipalEntityPath::Module(
                                `core::visual`,
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `bool`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::basic::bool`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::basic::bool`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 1,
                                                    ident_token: IdentToken {
                                                        ident: `bool`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 2,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::basic::bool`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 0,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::basic::bool`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `never`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::basic::never`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::basic::never`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 2,
                                                    ident_token: IdentToken {
                                                        ident: `never`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 3,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::basic::never`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 0,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::basic::never`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `unit`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::basic::unit`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::basic::unit`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 3,
                                                    ident_token: IdentToken {
                                                        ident: `unit`,
                                                        token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::basic::unit`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 0,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::basic::unit`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Trait`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::basic::Trait`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::basic::Trait`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 4,
                                                    ident_token: IdentToken {
                                                        ident: `Trait`,
                                                        token_idx: TokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::basic::Trait`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 0,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::basic::Trait`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::basic::Trait`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Module`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::basic::Module`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::basic::Module`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 5,
                                                    ident_token: IdentToken {
                                                        ident: `Module`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 6,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::basic::Module`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 0,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::basic::Module`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::basic::Module`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i8`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::i8`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::i8`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 44,
                                                    ident_token: IdentToken {
                                                        ident: `i8`,
                                                        token_idx: TokenIdx(
                                                            32,
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
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i8`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i8`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i16`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::i16`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::i16`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 49,
                                                    ident_token: IdentToken {
                                                        ident: `i16`,
                                                        token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 14,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i16`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i16`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i32`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::i32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 54,
                                                    ident_token: IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            158,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 15,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i32`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i64`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::i64`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::i64`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 59,
                                                    ident_token: IdentToken {
                                                        ident: `i64`,
                                                        token_idx: TokenIdx(
                                                            231,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 16,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i64`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i64`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i128`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::i128`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::i128`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 64,
                                                    ident_token: IdentToken {
                                                        ident: `i128`,
                                                        token_idx: TokenIdx(
                                                            283,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 17,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i128`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::i128`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `isize`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::isize`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 69,
                                                    ident_token: IdentToken {
                                                        ident: `isize`,
                                                        token_idx: TokenIdx(
                                                            335,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 18,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::isize`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::isize`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u8`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::u8`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::u8`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 74,
                                                    ident_token: IdentToken {
                                                        ident: `u8`,
                                                        token_idx: TokenIdx(
                                                            387,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 19,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::u8`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::u8`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u16`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::u16`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::u16`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 79,
                                                    ident_token: IdentToken {
                                                        ident: `u16`,
                                                        token_idx: TokenIdx(
                                                            439,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 20,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::u16`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::u16`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u32`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::u32`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::u32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 84,
                                                    ident_token: IdentToken {
                                                        ident: `u32`,
                                                        token_idx: TokenIdx(
                                                            491,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 21,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::u32`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::u32`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u64`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::u64`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::u64`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 89,
                                                    ident_token: IdentToken {
                                                        ident: `u64`,
                                                        token_idx: TokenIdx(
                                                            543,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 22,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::u64`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::u64`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u128`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::u128`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::u128`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 94,
                                                    ident_token: IdentToken {
                                                        ident: `u128`,
                                                        token_idx: TokenIdx(
                                                            595,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 23,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::u128`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::u128`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `usize`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::usize`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 99,
                                                    ident_token: IdentToken {
                                                        ident: `usize`,
                                                        token_idx: TokenIdx(
                                                            647,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 24,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `f32`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::f32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 104,
                                                    ident_token: IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            699,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::f32`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `f64`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::num::f64`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::num::f64`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 109,
                                                    ident_token: IdentToken {
                                                        ident: `f64`,
                                                        token_idx: TokenIdx(
                                                            812,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 26,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::f64`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 1,
                                            use_expr_idx: 3,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::f64`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `r32`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 8,
                                                    ident_token: IdentToken {
                                                        ident: `r32`,
                                                        token_idx: TokenIdx(
                                                            26,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 29,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 2,
                                            use_expr_idx: 6,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Debug`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::fmt::Debug`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::fmt::Debug`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 1,
                                                    ident_token: IdentToken {
                                                        ident: `Debug`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 6,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::fmt::Debug`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 3,
                                            use_expr_idx: 9,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::fmt::Debug`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::fmt::Debug`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Visualize`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::visual::Visualize`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::visual::Visualize`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 2,
                                                    ident_token: IdentToken {
                                                        ident: `Visualize`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 26,
                                                            },
                                                        ),
                                                        items: Some(
                                                            TraitItems {
                                                                ast_idx_range: ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::visual::Visualize`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 4,
                                            use_expr_idx: 12,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::visual::Visualize`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::visual::Visualize`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Html`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::visual::Html`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::visual::Html`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 3,
                                                    ident_token: IdentToken {
                                                        ident: `Html`,
                                                        token_idx: TokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 36,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::visual::Html`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 4,
                                            use_expr_idx: 12,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::visual::Html`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::visual::Html`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Copy`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::marker::Copy`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::marker::Copy`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 1,
                                                    ident_token: IdentToken {
                                                        ident: `Copy`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 7,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::marker::Copy`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 5,
                                            use_expr_idx: 15,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::marker::Copy`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::marker::Copy`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Sized`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::marker::Sized`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::marker::Sized`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 2,
                                                    ident_token: IdentToken {
                                                        ident: `Sized`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 8,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::marker::Sized`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 5,
                                            use_expr_idx: 15,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::marker::Sized`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::marker::Sized`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Clone`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::clone::Clone`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::clone::Clone`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 3,
                                                    ident_token: IdentToken {
                                                        ident: `Clone`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 1,
                                                            },
                                                        ),
                                                        items: Some(
                                                            TraitItems {
                                                                ast_idx_range: ArenaIdxRange(
                                                                    0..1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::clone::Clone`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 6,
                                            use_expr_idx: 18,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::clone::Clone`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::clone::Clone`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Option`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::option::Option`, `Enum`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::option::Option`, `Enum`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 4,
                                                    ident_token: IdentToken {
                                                        ident: `Option`,
                                                        token_idx: TokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 28,
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
                                                MajorItemPath::Type(
                                                    TypePath(`core::option::Option`, `Enum`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 7,
                                            use_expr_idx: 21,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::option::Option`, `Enum`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::option::Option`, `Enum`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Some`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::Use(
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
                                                    ast_idx: 3,
                                                    use_expr_idx: 2,
                                                },
                                            ),
                                            path: PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath {
                                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    ident: `Some`,
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 7,
                                            use_expr_idx: 21,
                                        },
                                    ),
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath {
                                            parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                            ident: `Some`,
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::TypeVariant(
                                TypeVariantPath {
                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                    ident: `Some`,
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `None`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::Use(
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
                                                    ast_idx: 3,
                                                    use_expr_idx: 2,
                                                },
                                            ),
                                            path: PrincipalEntityPath::TypeVariant(
                                                TypeVariantPath {
                                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    ident: `None`,
                                                },
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 7,
                                            use_expr_idx: 21,
                                        },
                                    ),
                                    path: PrincipalEntityPath::TypeVariant(
                                        TypeVariantPath {
                                            parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                            ident: `None`,
                                        },
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::TypeVariant(
                                TypeVariantPath {
                                    parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                    ident: `None`,
                                },
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `PartialEq`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::PartialEq`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::cmp::PartialEq`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 1,
                                                    ident_token: IdentToken {
                                                        ident: `PartialEq`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 2,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::PartialEq`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 8,
                                            use_expr_idx: 24,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::cmp::PartialEq`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::cmp::PartialEq`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Eq`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::Eq`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::cmp::Eq`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 2,
                                                    ident_token: IdentToken {
                                                        ident: `Eq`,
                                                        token_idx: TokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 3,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::Eq`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 8,
                                            use_expr_idx: 24,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::cmp::Eq`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::cmp::Eq`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `PartialOrd`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::PartialOrd`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::cmp::PartialOrd`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 3,
                                                    ident_token: IdentToken {
                                                        ident: `PartialOrd`,
                                                        token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::PartialOrd`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 8,
                                            use_expr_idx: 24,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::cmp::PartialOrd`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::cmp::PartialOrd`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Ord`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Trait(
                                                        TraitSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitPath(`core::cmp::Ord`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 4,
                                                    ident_token: IdentToken {
                                                        ident: `Ord`,
                                                        token_idx: TokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                    block: Trait {
                                                        path: TraitPath(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                        items: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::cmp::Ord`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 8,
                                            use_expr_idx: 24,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Trait(
                                            TraitPath(`core::cmp::Ord`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Trait(
                                    TraitPath(`core::cmp::Ord`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Slice`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::slice::Slice`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::slice::Slice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 9,
                                                    ident_token: IdentToken {
                                                        ident: `Slice`,
                                                        token_idx: TokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 32,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::slice::Slice`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 9,
                                            use_expr_idx: 27,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::slice::Slice`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::slice::Slice`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `CyclicSlice`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 12,
                                                    ident_token: IdentToken {
                                                        ident: `CyclicSlice`,
                                                        token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 33,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 9,
                                            use_expr_idx: 27,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Vec`,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::MajorItem {
                                                module_item_path: MajorItemPath::Type(
                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                ),
                                                node: MajorItemSynNode {
                                                    syn_node_path: MajorItemSynNodePath::Type(
                                                        TypeSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypePath(`core::vec::Vec`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 10,
                                                    ident_token: IdentToken {
                                                        ident: `Vec`,
                                                        token_idx: TokenIdx(
                                                            15,
                                                        ),
                                                    },
                                                    block: Type {
                                                        path: TypePath(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                        variants: None,
                                                    },
                                                },
                                            },
                                            path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 10,
                                            use_expr_idx: 30,
                                        },
                                    ),
                                    path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::vec::Vec`, `Extern`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `core`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: PrincipalEntityPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`core::vec::Vec`, `Extern`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `core::basic`,
                            ),
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
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
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
                            EntitySymbol::CrateRoot {
                                root_module_path: `core`,
                            },
                        ),
                    },
                },
            ],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [
                UseAllModuleSymbolsRule {
                    parent_module_path: `core`,
                    is_same_crate: true,
                    ast_idx: 0,
                    use_expr_idx: 0,
                    visibility: Scope::PubUnder(
                        `core::basic`,
                    ),
                    progress: Ok(
                        55,
                    ),
                },
            ],
        ),
        errors: [],
    },
)