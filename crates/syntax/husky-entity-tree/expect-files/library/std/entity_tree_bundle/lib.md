Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `std`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Ident(
                                "prelude",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 14,
                                    path: `std::prelude`,
                                    accessibility: Accessibility::PublicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Ident(
                                "logic",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 15,
                                    path: `std::logic`,
                                    accessibility: Accessibility::PublicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Ident(
                                "ops",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 16,
                                    path: `std::ops`,
                                    accessibility: Accessibility::PublicUnder(
                                        `std`,
                                    ),
                                    ast_idx: 2,
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
                            accessibility: AccessibilityProgress::Done {
                                accessibility: Accessibility::Public,
                            },
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
                            ident: Ident(
                                "Add",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `std::ops`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 53,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`std::ops::Add`),
                                    ),
                                    accessibility: Accessibility::PublicUnder(
                                        `std::ops`,
                                    ),
                                    ast_idx: 3,
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
        impl_blocks: ImplBlockBundle {
            all_ty_impl_blocks: [],
            all_ty_as_trai_impl_blocks: [],
            all_ill_formed_impl_blocks: [],
        },
    },
)