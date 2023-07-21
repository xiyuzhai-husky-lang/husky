Ok(
    EntitySynTreeSheet {
        module_path: `core::prelude`,
        major_entity_node_table: MajorEntityNodeTable {
            entries: [],
        },
        entity_symbol_table: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `bool`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::basic::bool`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `bool`,
                                        token_idx: TokenIdx(
                                            2,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::basic::bool`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `never`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::basic::never`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `never`,
                                        token_idx: TokenIdx(
                                            6,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::basic::never`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `unit`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::basic::unit`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `unit`,
                                        token_idx: TokenIdx(
                                            10,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::basic::unit`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Trait`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::basic::Trait`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::basic::Trait`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `Trait`,
                                        token_idx: TokenIdx(
                                            14,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::basic::Trait`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Module`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::basic::Module`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::basic::Module`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `Module`,
                                        token_idx: TokenIdx(
                                            18,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::basic::Module`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i8`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::i8`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 32,
                                    ident_token: IdentToken {
                                        ident: `i8`,
                                        token_idx: TokenIdx(
                                            28,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::i8`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i16`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::i16`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 37,
                                    ident_token: IdentToken {
                                        ident: `i16`,
                                        token_idx: TokenIdx(
                                            80,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::i16`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i32`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::i32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 42,
                                    ident_token: IdentToken {
                                        ident: `i32`,
                                        token_idx: TokenIdx(
                                            132,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i64`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::i64`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 47,
                                    ident_token: IdentToken {
                                        ident: `i64`,
                                        token_idx: TokenIdx(
                                            183,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::i64`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `i128`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::i128`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 52,
                                    ident_token: IdentToken {
                                        ident: `i128`,
                                        token_idx: TokenIdx(
                                            235,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::i128`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `isize`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::isize`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 57,
                                    ident_token: IdentToken {
                                        ident: `isize`,
                                        token_idx: TokenIdx(
                                            287,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::isize`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u8`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::u8`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 62,
                                    ident_token: IdentToken {
                                        ident: `u8`,
                                        token_idx: TokenIdx(
                                            339,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::u8`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u16`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::u16`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 67,
                                    ident_token: IdentToken {
                                        ident: `u16`,
                                        token_idx: TokenIdx(
                                            391,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::u16`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u32`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::u32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 72,
                                    ident_token: IdentToken {
                                        ident: `u32`,
                                        token_idx: TokenIdx(
                                            443,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::u32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u64`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::u64`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 77,
                                    ident_token: IdentToken {
                                        ident: `u64`,
                                        token_idx: TokenIdx(
                                            495,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::u64`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `u128`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::u128`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 82,
                                    ident_token: IdentToken {
                                        ident: `u128`,
                                        token_idx: TokenIdx(
                                            547,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::u128`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `usize`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::usize`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 87,
                                    ident_token: IdentToken {
                                        ident: `usize`,
                                        token_idx: TokenIdx(
                                            599,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::usize`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `f32`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::f32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 92,
                                    ident_token: IdentToken {
                                        ident: `f32`,
                                        token_idx: TokenIdx(
                                            651,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::f32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `f64`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::num::f64`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 97,
                                    ident_token: IdentToken {
                                        ident: `f64`,
                                        token_idx: TokenIdx(
                                            710,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 27,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::num::f64`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 1,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `r32`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `r32`,
                                        token_idx: TokenIdx(
                                            22,
                                        ),
                                    },
                                    block: Type {
                                        path: TypePath(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                        variants: None,
                                    },
                                },
                            },
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::raw_bits::r32`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 2,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Debug`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::fmt::Debug`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::fmt::Debug`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `Debug`,
                                        token_idx: TokenIdx(
                                            2,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::fmt::Debug`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 3,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Visualize`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::visual::Visualize`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::visual::Visualize`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `Visualize`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                    block: Trait {
                                        path: TraitPath(
                                            Id {
                                                value: 25,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::visual::Visualize`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Html`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::visual::Html`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::visual::Html`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `Html`,
                                        token_idx: TokenIdx(
                                            13,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::visual::Html`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 4,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Copy`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::marker::Copy`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::marker::Copy`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `Copy`,
                                        token_idx: TokenIdx(
                                            2,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::marker::Copy`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            use_expr_idx: 15,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Sized`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::marker::Sized`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::marker::Sized`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `Sized`,
                                        token_idx: TokenIdx(
                                            6,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::marker::Sized`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 5,
                            use_expr_idx: 15,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Clone`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::clone::Clone`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::clone::Clone`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `Clone`,
                                        token_idx: TokenIdx(
                                            2,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::clone::Clone`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 6,
                            use_expr_idx: 18,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Option`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::option::Option`, `Enum`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 7,
                            use_expr_idx: 21,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Some`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 2,
                                    use_expr_idx: 0,
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
                },
                EntitySymbolEntry {
                    ident: `None`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
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
                                    ast_idx: 2,
                                    use_expr_idx: 0,
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
                },
                EntitySymbolEntry {
                    ident: `PartialEq`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::PartialEq`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::cmp::PartialEq`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `PartialEq`,
                                        token_idx: TokenIdx(
                                            2,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::PartialEq`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 8,
                            use_expr_idx: 24,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Eq`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::Eq`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::cmp::Eq`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `Eq`,
                                        token_idx: TokenIdx(
                                            6,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::Eq`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 8,
                            use_expr_idx: 24,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `PartialOrd`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::PartialOrd`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::cmp::PartialOrd`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `PartialOrd`,
                                        token_idx: TokenIdx(
                                            10,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::PartialOrd`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 8,
                            use_expr_idx: 24,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Ord`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::Ord`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Trait(
                                        TraitSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`core::cmp::Ord`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `Ord`,
                                        token_idx: TokenIdx(
                                            14,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Trait(
                                    TraitPath(`core::cmp::Ord`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 8,
                            use_expr_idx: 24,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `Slice`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::slice::Slice`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::slice::Slice`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `Slice`,
                                        token_idx: TokenIdx(
                                            2,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::slice::Slice`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 9,
                            use_expr_idx: 27,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `CyclicSliceLeashed`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem {
                                module_item_path: ModuleItemPath::Type(
                                    TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                ),
                                node: ModuleItemSynNode {
                                    syn_node_path: ModuleItemSynNodePath::Type(
                                        TypeSynNodePath {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    visibility: Scope::Pub,
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `CyclicSliceLeashed`,
                                        token_idx: TokenIdx(
                                            10,
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
                            path: PrincipalEntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                                ),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 9,
                            use_expr_idx: 27,
                        },
                    ),
                },
            ],
        ),
        impl_block_node_table: [],
        once_use_rules: OnceUseRules(
            [
                OnceUseRule {
                    ast_idx: 0,
                    use_expr_idx: 2,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
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
                            EntitySymbol::CrateRoot {
                                root_module_path: `core`,
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 1,
                    use_expr_idx: 5,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
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
                OnceUseRule {
                    ast_idx: 2,
                    use_expr_idx: 8,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    16,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
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
                OnceUseRule {
                    ast_idx: 3,
                    use_expr_idx: 11,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            10..11,
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
                OnceUseRule {
                    ast_idx: 4,
                    use_expr_idx: 14,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    30,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            13..14,
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
                OnceUseRule {
                    ast_idx: 5,
                    use_expr_idx: 17,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    37,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            16..17,
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
                OnceUseRule {
                    ast_idx: 6,
                    use_expr_idx: 20,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    44,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            19..20,
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
                OnceUseRule {
                    ast_idx: 7,
                    use_expr_idx: 23,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            22..23,
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
                OnceUseRule {
                    ast_idx: 8,
                    use_expr_idx: 26,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    58,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            25..26,
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
                OnceUseRule {
                    ast_idx: 9,
                    use_expr_idx: 29,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::CrateRoot(
                            CrateToken {
                                token_idx: TokenIdx(
                                    65,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            28..29,
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
                OnceUseRule {
                    ast_idx: 0,
                    use_expr_idx: 1,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `basic`,
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
                            EntitySymbol::Submodule {
                                submodule_path: `core::basic`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::basic`,
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
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 1,
                    use_expr_idx: 4,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `num`,
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            3..4,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::num`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::num`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 10,
                                    ident_token: IdentToken {
                                        ident: `num`,
                                        token_idx: TokenIdx(
                                            32,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 2,
                    use_expr_idx: 7,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `raw_bits`,
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::raw_bits`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::raw_bits`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 14,
                                    ident_token: IdentToken {
                                        ident: `raw_bits`,
                                        token_idx: TokenIdx(
                                            44,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 3,
                    use_expr_idx: 10,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `fmt`,
                                token_idx: TokenIdx(
                                    25,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::fmt`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::fmt`,
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
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 4,
                    use_expr_idx: 13,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `visual`,
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            12..13,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::visual`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::visual`,
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
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 5,
                    use_expr_idx: 16,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `marker`,
                                token_idx: TokenIdx(
                                    39,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            15..16,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::marker`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::marker`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 8,
                                    ident_token: IdentToken {
                                        ident: `marker`,
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 6,
                    use_expr_idx: 19,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `clone`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            18..19,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::clone`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::clone`,
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
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 7,
                    use_expr_idx: 22,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `option`,
                                token_idx: TokenIdx(
                                    53,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            21..22,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::option`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::option`,
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::Pub,
                                    ast_idx: 12,
                                    ident_token: IdentToken {
                                        ident: `option`,
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                },
                            },
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 8,
                    use_expr_idx: 25,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `cmp`,
                                token_idx: TokenIdx(
                                    60,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            24..25,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::cmp`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::cmp`,
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
                        ),
                    },
                },
                OnceUseRule {
                    ast_idx: 9,
                    use_expr_idx: 28,
                    visibility: Scope::Pub,
                    variant: OnceUseRuleVariant::Parent {
                        parent_name_token: PathNameToken::Ident(
                            IdentToken {
                                ident: `slice`,
                                token_idx: TokenIdx(
                                    67,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            27..28,
                        ),
                    },
                    parent: Some(
                        MajorEntityPath::Module(
                            `core`,
                        ),
                    ),
                    state: OnceUseRuleState::Resolved {
                        original_symbol: Some(
                            EntitySymbol::Submodule {
                                submodule_path: `core::slice`,
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: `core::slice`,
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
                        ),
                    },
                },
            ],
        ),
        use_all_rules: UseAllModuleSymbolsRules(
            [
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::basic`,
                    is_same_crate: true,
                    ast_idx: 0,
                    use_expr_idx: 0,
                    visibility: Scope::Pub,
                    progress: Ok(
                        5,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::num`,
                    is_same_crate: true,
                    ast_idx: 1,
                    use_expr_idx: 3,
                    visibility: Scope::Pub,
                    progress: Ok(
                        15,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::raw_bits`,
                    is_same_crate: true,
                    ast_idx: 2,
                    use_expr_idx: 6,
                    visibility: Scope::Pub,
                    progress: Ok(
                        1,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::fmt`,
                    is_same_crate: true,
                    ast_idx: 3,
                    use_expr_idx: 9,
                    visibility: Scope::Pub,
                    progress: Ok(
                        1,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::visual`,
                    is_same_crate: true,
                    ast_idx: 4,
                    use_expr_idx: 12,
                    visibility: Scope::Pub,
                    progress: Ok(
                        2,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::marker`,
                    is_same_crate: true,
                    ast_idx: 5,
                    use_expr_idx: 15,
                    visibility: Scope::Pub,
                    progress: Ok(
                        2,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::clone`,
                    is_same_crate: true,
                    ast_idx: 6,
                    use_expr_idx: 18,
                    visibility: Scope::Pub,
                    progress: Ok(
                        1,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::option`,
                    is_same_crate: true,
                    ast_idx: 7,
                    use_expr_idx: 21,
                    visibility: Scope::Pub,
                    progress: Ok(
                        3,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::cmp`,
                    is_same_crate: true,
                    ast_idx: 8,
                    use_expr_idx: 24,
                    visibility: Scope::Pub,
                    progress: Ok(
                        4,
                    ),
                },
                UseAllModuleSymbolsRule {
                    parent_module_path: `core::slice`,
                    is_same_crate: true,
                    ast_idx: 9,
                    use_expr_idx: 27,
                    visibility: Scope::Pub,
                    progress: Ok(
                        2,
                    ),
                },
            ],
        ),
        errors: [],
    },
)