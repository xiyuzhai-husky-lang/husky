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
                                    path: TypePath(`core::basic::bool`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Trait`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::basic::Trait`, `Structure`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Module`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::basic::Module`, `Structure`),
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
                            ident: `LogicAnd`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::logic::LogicAnd`, `Structure`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LogicOr`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::logic::LogicOr`, `Inductive`),
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
                                    path: TypePath(`core::num::i8`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i16`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i16`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i32`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i32`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i64`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::i64`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f8`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f8`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f16`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f16`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f32`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f32`, `Foreign`),
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f64`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::num::f64`, `Foreign`),
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
                                            path: TypePath(`core::basic::bool`, `Foreign`),
                                        },
                                    ),
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
                                        },
                                    ),
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
                                        },
                                    ),
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
                                            path: TypePath(`core::num::i8`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::i16`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::i32`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::i64`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::f8`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::f16`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::f32`, `Foreign`),
                                        },
                                    ),
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
                                            path: TypePath(`core::num::f64`, `Foreign`),
                                        },
                                    ),
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)