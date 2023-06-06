Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `prelude`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `std::prelude`,
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
                        },
                        EntitySymbolEntry {
                            ident: `logic`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `std::logic`,
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
                        },
                        EntitySymbolEntry {
                            ident: `ops`,
                            visibility: Scope::PubUnder(
                                `std`,
                            ),
                            symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `std::ops`,
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
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `std::prelude`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `std::logic`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 0,
                            use_expr_idx: 2,
                            visibility: Scope::Pub,
                            variant: UseExprRuleVariant::Parent {
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
                            state: UseExprRuleState::Erroneous,
                        },
                    ],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `core`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `std::ops`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Add`,
                            visibility: Scope::PubUnder(
                                `std::ops`,
                            ),
                            symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`std::ops::Add`),
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
                                },
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
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