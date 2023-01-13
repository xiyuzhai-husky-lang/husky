Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::raw_contour`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `RawContour`,
                    accessibility: PubicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            accessibility: PubicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 203,
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
                            ast_idx: 205,
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
                            ast_idx: 210,
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
                            ast_idx: 211,
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
                            ast_idx: 212,
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
                            ast_idx: 213,
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
                            ast_idx: 214,
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
                            ast_idx: 215,
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
                            ast_idx: 216,
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
                            ast_idx: 217,
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
                            ast_idx: 218,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 199,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            1..2,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 200,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            4..5,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 201,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            7..8,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 202,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    use_expr_children: Some(
                        ArenaIdxRange(
                            9..11,
                        ),
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)