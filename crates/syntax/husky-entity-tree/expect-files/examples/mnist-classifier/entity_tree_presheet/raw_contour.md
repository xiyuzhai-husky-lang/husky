Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::raw_contour`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `RawContour`,
                    accessibility: Public,
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            accessibility: Public,
                            ast_idx: 199,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `Direction`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_pixel_pair`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 206,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_pixel_to_the_left`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 207,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_pixel_to_the_right`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 208,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_inward_direction`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 209,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_angle_change`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 210,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_outward_direction`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 211,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `StreakCache`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 212,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `get_concave_middle_point`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 213,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `find_raw_contours`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 214,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 195,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 196,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 197,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 198,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 83,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..11,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)