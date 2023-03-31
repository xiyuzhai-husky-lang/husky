Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `prelude`,
                            visibility: Visibility::PubUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `logic`,
                            visibility: Visibility::PubUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 32,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ops`,
                            visibility: Visibility::PubUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 33,
                                    },
                                ),
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
                            visibility: Visibility::Pub,
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
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
                            visibility: Visibility::PubUnder(
                                `std::ops`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 107,
                                    },
                                ),
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