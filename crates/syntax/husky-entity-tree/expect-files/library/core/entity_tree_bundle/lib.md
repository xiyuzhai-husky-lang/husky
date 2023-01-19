Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `core`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `basic`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::basic`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `logic`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::logic`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `num`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::num`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ops`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::ops`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `prelude`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::prelude`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 4,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_bits`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::raw_bits`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 5,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::basic`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `bool`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::basic::bool`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Trait`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::basic::Trait`, `Structure`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Module`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::basic::Module`, `Structure`),
                                    accessibility: Public,
                                    ast_idx: 2,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::logic`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Prop`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::logic::Prop`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LogicAnd`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LogicOr`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::logic::LogicOr`, `Inductive`),
                                    accessibility: Public,
                                    ast_idx: 2,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::num`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `i8`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i8`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 17,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i16`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i16`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 21,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i32`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 25,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i64`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i64`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 29,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f8`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f8`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 33,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f16`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f16`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 37,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f32`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 41,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f64`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f64`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 45,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::ops`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Add`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::ops::Add`),
                                    accessibility: Public,
                                    ast_idx: 3,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::prelude`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `bool`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::basic::bool`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: TypePath(`core::basic::bool`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Trait`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::basic::Trait`, `Structure`),
                                            accessibility: Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: TypePath(`core::basic::Trait`, `Structure`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Module`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::basic::Module`, `Structure`),
                                            accessibility: Public,
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: TypePath(`core::basic::Module`, `Structure`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i8`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::i8`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 17,
                                        },
                                    ),
                                    path: TypePath(`core::num::i8`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i16`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::i16`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 21,
                                        },
                                    ),
                                    path: TypePath(`core::num::i16`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i32`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::i32`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 25,
                                        },
                                    ),
                                    path: TypePath(`core::num::i32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i64`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::i64`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 29,
                                        },
                                    ),
                                    path: TypePath(`core::num::i64`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f8`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::f8`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 33,
                                        },
                                    ),
                                    path: TypePath(`core::num::f8`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f16`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::f16`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 37,
                                        },
                                    ),
                                    path: TypePath(`core::num::f16`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f32`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::f32`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 41,
                                        },
                                    ),
                                    path: TypePath(`core::num::f32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f64`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::num::f64`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 45,
                                        },
                                    ),
                                    path: TypePath(`core::num::f64`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `r32`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TypePath(`core::raw_bits::r32`, `Alien`),
                                            accessibility: Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: TypePath(`core::raw_bits::r32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 2,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::raw_bits`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `r32`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::raw_bits::r32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)