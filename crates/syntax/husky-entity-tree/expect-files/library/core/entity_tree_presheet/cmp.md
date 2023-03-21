Ok(
    EntityTreePresheet {
        module_path: `core::cmp`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `PartialEq`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::cmp::PartialEq`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `PartialEq`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Eq`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::cmp::Eq`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `Eq`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `PartialOrd`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::cmp::PartialOrd`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `PartialOrd`,
                                token_idx: TokenIdx(
                                    10,
                                ),
                            },
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Ord`,
                    accessibility: Visibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                            visibility: Visibility::Public,
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `Ord`,
                                token_idx: TokenIdx(
                                    14,
                                ),
                            },
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)