Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::fermi`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `FermiMatchResult`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 22,
                            ident_token: IdentToken {
                                ident: `FermiMatchResult`,
                                token_idx: TokenIdx(
                                    6,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi_match`,
                    visibility: Scope::Pub,
                    symbol: EntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 24,
                            ident_token: IdentToken {
                                ident: `fermi_match`,
                                token_idx: TokenIdx(
                                    147,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::connected_component`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 11,
                                    ident_token: IdentToken {
                                        ident: `connected_component`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::connected_component`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::raw_contour`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 12,
                                    ident_token: IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::raw_contour`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::geom2d`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 13,
                                    ident_token: IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::geom2d`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 14,
                                    ident_token: IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::fermi`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 15,
                                    ident_token: IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::fermi`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `digits`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 16,
                                    ident_token: IdentToken {
                                        ident: `digits`,
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::digits`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::major`,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 17,
                                    ident_token: IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::Module(
                                `mnist_classifier::major`,
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `narrow_down`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`malamute::narrow_down`, `Gn`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 0,
                                            ident_token: IdentToken {
                                                ident: `narrow_down`,
                                                token_idx: TokenIdx(
                                                    2,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`malamute::narrow_down`, `Gn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 24,
                                    use_expr_idx: 18,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `MnistLabel`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 10,
                                            ident_token: IdentToken {
                                                ident: `MnistLabel`,
                                                token_idx: TokenIdx(
                                                    2,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 25,
                                    use_expr_idx: 20,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `BinaryImage28`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist::BinaryImage28`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 11,
                                            ident_token: IdentToken {
                                                ident: `BinaryImage28`,
                                                token_idx: TokenIdx(
                                                    25,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 25,
                                    use_expr_idx: 20,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist::BinaryImage28`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 19,
                                            ident_token: IdentToken {
                                                ident: `connected_components`,
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_connected_component`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 20,
                                            ident_token: IdentToken {
                                                ident: `major_connected_component`,
                                                token_idx: TokenIdx(
                                                    19,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 21,
                                            ident_token: IdentToken {
                                                ident: `ignored_connected_components_row_span_sum_sum`,
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contours`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contours`,
                                                token_idx: TokenIdx(
                                                    104,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 23,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contour`,
                                                token_idx: TokenIdx(
                                                    116,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_line_segment_sketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `major_line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 25,
                                            ident_token: IdentToken {
                                                ident: `major_concave_components`,
                                                token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_one`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::ModuleItem(
                                                ModuleItemSymbol {
                                                    path: ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 63,
                                                    ident_token: IdentToken {
                                                        ident: `is_one`,
                                                        token_idx: TokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                },
                                            ),
                                            path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 10,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 19,
                                    use_expr_idx: 3,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 203,
                                            ident_token: IdentToken {
                                                ident: `RawContour`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 9,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_raw_contours`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 215,
                                            ident_token: IdentToken {
                                                ident: `find_raw_contours`,
                                                token_idx: TokenIdx(
                                                    954,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 21,
                                    use_expr_idx: 9,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 169,
                                            ident_token: IdentToken {
                                                ident: `LineSegmentStroke`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 172,
                                            ident_token: IdentToken {
                                                ident: `LineSegmentSketch`,
                                                token_idx: TokenIdx(
                                                    173,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::ModuleItem(
                                                ModuleItemSymbol {
                                                    path: ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 74,
                                                    ident_token: IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                },
                                            ),
                                            path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::ModuleItem(
                                                ModuleItemSymbol {
                                                    path: ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 77,
                                                    ident_token: IdentToken {
                                                        ident: `find_concave_components`,
                                                        token_idx: TokenIdx(
                                                            541,
                                                        ),
                                                    },
                                                },
                                            ),
                                            path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::Use(
                                        UseSymbol {
                                            original_symbol: EntitySymbol::ModuleItem(
                                                ModuleItemSymbol {
                                                    path: ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                    ),
                                                    visibility: Scope::Pub,
                                                    ast_idx: 16,
                                                    ident_token: IdentToken {
                                                        ident: `LineSegment`,
                                                        token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            ),
                                            path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                ),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 165,
                                            use_expr_idx: 4,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 120,
                                            ident_token: IdentToken {
                                                ident: `ConnectedComponentDistribution`,
                                                token_idx: TokenIdx(
                                                    12,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 15,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 121,
                                            ident_token: IdentToken {
                                                ident: `EffHoles`,
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 15,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 123,
                                            ident_token: IdentToken {
                                                ident: `ConnectedComponent`,
                                                token_idx: TokenIdx(
                                                    71,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 15,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                            ),
                                            visibility: Scope::Pub,
                                            ast_idx: 127,
                                            ident_token: IdentToken {
                                                ident: `find_connected_components`,
                                                token_idx: TokenIdx(
                                                    654,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 15,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                            ),
                            visibility: Scope::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [
            ImplBlock::Type(
                TypeImplBlock {
                    id: TypeImplBlockId {
                        module_path: `mnist_classifier::fermi`,
                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        disambiguator: 0,
                    },
                    ast_idx: 23,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                    ty_expr: 23,
                    body: Type(
                        TypeItems {
                            ast_idx_range: ArenaIdxRange(
                                12..15,
                            ),
                        },
                    ),
                },
            ),
        ],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 1,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    },
                },
            ],
        ),
        use_all_rules: UseAllRules(
            [
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier`,
                    },
                    ast_idx: 21,
                    use_expr_idx: 0,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::fermi`,
                    ),
                    progress: Ok(
                        31,
                    ),
                },
            ],
        ),
        errors: [],
    },
)