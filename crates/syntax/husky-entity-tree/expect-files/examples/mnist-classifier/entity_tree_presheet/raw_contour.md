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