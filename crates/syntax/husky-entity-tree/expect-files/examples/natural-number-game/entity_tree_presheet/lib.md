Ok(
    EntityTreePresheet {
        module_path: `natural_number_game`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "Nat",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `natural_number_game`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`natural_number_game::Nat`, `Inductive`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 3,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "OddNat",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `natural_number_game`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`natural_number_game::OddNat`, `Structure`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 9,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "EvenNat",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `natural_number_game`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`natural_number_game::EvenNat`, `Structure`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `natural_number_game`,
                            ),
                            ast_idx: 10,
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