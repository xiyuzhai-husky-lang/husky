Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch`,
        module_specific_symbols: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `concave_component`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::concave_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 159,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `convex_component`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convex_component`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 160,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `convexity`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::convexity`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 161,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `line_segment`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch::line_segment`,
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 162,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 168,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 170,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `go_right`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 172,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `go_left`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 173,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `extend_end`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 174,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `extend_start`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 175,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: `find_line_segments`,
                    accessibility: PubicUnder(
                        `mnist_classifier::line_segment_sketch`,
                    ),
                    symbol: ModuleItem(
                        ModuleItemSymbol {
                            path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
                            accessibility: PubicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            ast_idx: 176,
                        },
                    ),
                },
            ],
        ),
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 163,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    9,
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
                    ast_idx: 164,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
                        ),
                    },
                    variant: Parent {
                        parent_name_token: Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    15,
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
                    ast_idx: 165,
                    accessibility: Done {
                        accessibility: Public,
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 80,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 166,
                    accessibility: Done {
                        accessibility: Public,
                    },
                    variant: Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 81,
                                        },
                                    ),
                                ),
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            8..9,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 167,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch`,
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
                                    31,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            10..11,
                        ),
                    },
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)