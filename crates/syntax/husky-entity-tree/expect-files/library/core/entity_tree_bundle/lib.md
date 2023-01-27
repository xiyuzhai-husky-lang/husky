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
                        EntitySymbolEntry {
                            ident: `fmt`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::fmt`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `clone`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::clone`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 7,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `marker`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::marker`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 8,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `vec`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::vec`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `cmp`,
                            accessibility: PubicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    path: `core::cmp`,
                                    accessibility: PubicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 10,
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
                                    ast_idx: 18,
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
                                    ast_idx: 23,
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
                                    ast_idx: 28,
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
                                    ast_idx: 33,
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
                                    ast_idx: 38,
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
                                    ast_idx: 43,
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
                                    ast_idx: 48,
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
                                    ast_idx: 53,
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
                                            ast_idx: 18,
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
                                            ast_idx: 23,
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
                                            ast_idx: 28,
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
                                            ast_idx: 33,
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
                                            ast_idx: 38,
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
                                            ast_idx: 43,
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
                                            ast_idx: 48,
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
                                            ast_idx: 53,
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
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: TypePath(`core::raw_bits::r32`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 2,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Debug`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::fmt::Debug`),
                                            accessibility: Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: TraitPath(`core::fmt::Debug`),
                                    accessibility: Public,
                                    ast_idx: 3,
                                    use_expr_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Copy`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::marker::Copy`),
                                            accessibility: Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: TraitPath(`core::marker::Copy`),
                                    accessibility: Public,
                                    ast_idx: 4,
                                    use_expr_idx: 12,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Sized`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::marker::Sized`),
                                            accessibility: Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: TraitPath(`core::marker::Sized`),
                                    accessibility: Public,
                                    ast_idx: 4,
                                    use_expr_idx: 12,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Clone`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::clone::Clone`),
                                            accessibility: Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: TraitPath(`core::clone::Clone`),
                                    accessibility: Public,
                                    ast_idx: 5,
                                    use_expr_idx: 15,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `PartialEq`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::cmp::PartialEq`),
                                            accessibility: Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: TraitPath(`core::cmp::PartialEq`),
                                    accessibility: Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Eq`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::cmp::Eq`),
                                            accessibility: Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: TraitPath(`core::cmp::Eq`),
                                    accessibility: Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `PartialOrd`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::cmp::PartialOrd`),
                                            accessibility: Public,
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: TraitPath(`core::cmp::PartialOrd`),
                                    accessibility: Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Ord`,
                            accessibility: Public,
                            symbol: Use(
                                UseSymbol {
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            path: TraitPath(`core::cmp::Ord`),
                                            accessibility: Public,
                                            ast_idx: 3,
                                        },
                                    ),
                                    path: TraitPath(`core::cmp::Ord`),
                                    accessibility: Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
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
                                    ast_idx: 2,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::fmt`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Debug`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::fmt::Debug`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::clone`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Clone`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::clone::Clone`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::marker`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Copy`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::marker::Copy`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Sized`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::marker::Sized`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::vec`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Vec`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TypePath(`core::vec::Vec`, `Alien`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                    ],
                ),
            },
            EntityTreeSheet {
                module_path: `core::cmp`,
                module_specific_symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `PartialEq`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::cmp::PartialEq`),
                                    accessibility: Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Eq`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::cmp::Eq`),
                                    accessibility: Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `PartialOrd`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::cmp::PartialOrd`),
                                    accessibility: Public,
                                    ast_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Ord`,
                            accessibility: Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    path: TraitPath(`core::cmp::Ord`),
                                    accessibility: Public,
                                    ast_idx: 3,
                                },
                            ),
                        },
                    ],
                ),
            },
        ],
    },
)