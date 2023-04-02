Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier::digits`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `zero`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::zero`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `one`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 1,
                            ident_token: IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `six`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::six`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 2,
                            ident_token: IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `three`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::three`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 3,
                            ident_token: IdentToken {
                                ident: `three`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `four`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::four`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 4,
                            ident_token: IdentToken {
                                ident: `four`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `five`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::five`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 5,
                            ident_token: IdentToken {
                                ident: `five`,
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `seven`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::seven`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 6,
                            ident_token: IdentToken {
                                ident: `seven`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `eight`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::eight`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 7,
                            ident_token: IdentToken {
                                ident: `eight`,
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `nine`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::nine`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 8,
                            ident_token: IdentToken {
                                ident: `nine`,
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `two`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::two`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 9,
                            ident_token: IdentToken {
                                ident: `two`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_one`,
                    visibility: Visibility::Pub,
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 63,
                                    ident_token: IdentToken {
                                        ident: `is_one`,
                                        token_idx: TokenIdx(
                                            24,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                            ),
                            visibility: Visibility::Pub,
                            ast_idx: 10,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::connected_component`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::raw_contour`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::geom2d`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::line_segment_sketch`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::fermi`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `digits`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::digits`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Submodule(
                                SubmoduleSymbol {
                                    path: `mnist_classifier::major`,
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `MnistLabel`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
                                            ast_idx: 0,
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
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 24,
                                    use_expr_idx: 18,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_six`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 52,
                                    ident_token: IdentToken {
                                        ident: `is_six`,
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 11,
                            use_expr_idx: 2,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_zero`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 28,
                                    ident_token: IdentToken {
                                        ident: `is_zero`,
                                        token_idx: TokenIdx(
                                            50,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 12,
                            use_expr_idx: 5,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_two`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 54,
                                    ident_token: IdentToken {
                                        ident: `is_two`,
                                        token_idx: TokenIdx(
                                            114,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 13,
                            use_expr_idx: 8,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_three`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 29,
                                    ident_token: IdentToken {
                                        ident: `is_three`,
                                        token_idx: TokenIdx(
                                            27,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 14,
                            use_expr_idx: 11,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_five`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `is_five`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 15,
                            use_expr_idx: 14,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_seven`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 45,
                                    ident_token: IdentToken {
                                        ident: `is_seven`,
                                        token_idx: TokenIdx(
                                            166,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 16,
                            use_expr_idx: 17,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_eight`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 19,
                                    ident_token: IdentToken {
                                        ident: `is_eight`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 17,
                            use_expr_idx: 20,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_nine`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier::digits`,
                                    ),
                                    ast_idx: 38,
                                    ident_token: IdentToken {
                                        ident: `is_nine`,
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 23,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
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
                                            FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 20,
                                            ident_token: IdentToken {
                                                ident: `major_connected_component`,
                                                token_idx: TokenIdx(
                                                    20,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 21,
                                            ident_token: IdentToken {
                                                ident: `ignored_connected_components_row_span_sum_sum`,
                                                token_idx: TokenIdx(
                                                    72,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contours`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contours`,
                                                token_idx: TokenIdx(
                                                    107,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 23,
                                            ident_token: IdentToken {
                                                ident: `major_raw_contour`,
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `major_line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_concave_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 25,
                                            ident_token: IdentToken {
                                                ident: `major_concave_components`,
                                                token_idx: TokenIdx(
                                                    145,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 18,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `FermiMatchResult`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 22,
                                            ident_token: IdentToken {
                                                ident: `FermiMatchResult`,
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 20,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi_match`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 24,
                                            ident_token: IdentToken {
                                                ident: `fermi_match`,
                                                token_idx: TokenIdx(
                                                    150,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 20,
                                    use_expr_idx: 6,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
                                            ast_idx: 199,
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
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
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
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
                                            ast_idx: 171,
                                            ident_token: IdentToken {
                                                ident: `LineSegmentSketch`,
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                                    visibility: Visibility::Pub,
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
                                            visibility: Visibility::Pub,
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                    ),
                                                    visibility: Visibility::Pub,
                                                    ast_idx: 76,
                                                    ident_token: IdentToken {
                                                        ident: `find_concave_components`,
                                                        token_idx: TokenIdx(
                                                            522,
                                                        ),
                                                    },
                                                },
                                            ),
                                            path: EntityPath::ModuleItem(
                                                ModuleItemPath::Form(
                                                    FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                                ),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 163,
                                            use_expr_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 22,
                                    use_expr_idx: 12,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                                    visibility: Visibility::Pub,
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
                                            visibility: Visibility::Pub,
                                            ast_idx: 165,
                                            use_expr_idx: 4,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
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
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
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
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
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
                                            visibility: Visibility::Pub,
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
                                    visibility: Visibility::PubUnder(
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
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 126,
                                            ident_token: IdentToken {
                                                ident: `find_connected_components`,
                                                token_idx: TokenIdx(
                                                    646,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Visibility::PubUnder(
                                        `mnist_classifier`,
                                    ),
                                    ast_idx: 23,
                                    use_expr_idx: 15,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 26,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 10,
                    use_expr_idx: 1,
                    visibility: Visibility::Pub,
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::one`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 1,
                                ident_token: IdentToken {
                                    ident: `one`,
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 11,
                    use_expr_idx: 4,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    26,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            3..4,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 12,
                    use_expr_idx: 7,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 13,
                    use_expr_idx: 10,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 14,
                    use_expr_idx: 13,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    44,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            12..13,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 15,
                    use_expr_idx: 16,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    50,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            15..16,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 16,
                    use_expr_idx: 19,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    56,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            18..19,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 17,
                    use_expr_idx: 22,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    62,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            21..22,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 25,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    68,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            24..25,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier::digits`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 27,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Crate(
                            CrateToken {
                                token_idx: TokenIdx(
                                    74,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            26..27,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 10,
                    use_expr_idx: 0,
                    visibility: Visibility::Pub,
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_one`,
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::one`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                                visibility: Visibility::Pub,
                                ast_idx: 63,
                                ident_token: IdentToken {
                                    ident: `is_one`,
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 11,
                    use_expr_idx: 3,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    28,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            2..3,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::six`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `six`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 12,
                    use_expr_idx: 6,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            5..6,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::zero`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 0,
                                ident_token: IdentToken {
                                    ident: `zero`,
                                    token_idx: TokenIdx(
                                        1,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 13,
                    use_expr_idx: 9,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `two`,
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            8..9,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::two`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 9,
                                ident_token: IdentToken {
                                    ident: `two`,
                                    token_idx: TokenIdx(
                                        19,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 14,
                    use_expr_idx: 12,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `three`,
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            11..12,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::three`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 3,
                                ident_token: IdentToken {
                                    ident: `three`,
                                    token_idx: TokenIdx(
                                        7,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 15,
                    use_expr_idx: 15,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `five`,
                                token_idx: TokenIdx(
                                    52,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            14..15,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::five`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 5,
                                ident_token: IdentToken {
                                    ident: `five`,
                                    token_idx: TokenIdx(
                                        11,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 16,
                    use_expr_idx: 18,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `seven`,
                                token_idx: TokenIdx(
                                    58,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            17..18,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::seven`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 6,
                                ident_token: IdentToken {
                                    ident: `seven`,
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 17,
                    use_expr_idx: 21,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `eight`,
                                token_idx: TokenIdx(
                                    64,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            20..21,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::eight`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 7,
                                ident_token: IdentToken {
                                    ident: `eight`,
                                    token_idx: TokenIdx(
                                        15,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 24,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `nine`,
                                token_idx: TokenIdx(
                                    70,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            23..24,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits::nine`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 8,
                                ident_token: IdentToken {
                                    ident: `nine`,
                                    token_idx: TokenIdx(
                                        17,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 11,
                    use_expr_idx: 2,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_six`,
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::six`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::six::is_six`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 52,
                                ident_token: IdentToken {
                                    ident: `is_six`,
                                    token_idx: TokenIdx(
                                        39,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 12,
                    use_expr_idx: 5,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_zero`,
                            token_idx: TokenIdx(
                                36,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::zero`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 28,
                                ident_token: IdentToken {
                                    ident: `is_zero`,
                                    token_idx: TokenIdx(
                                        50,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 13,
                    use_expr_idx: 8,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_two`,
                            token_idx: TokenIdx(
                                42,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::two`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 54,
                                ident_token: IdentToken {
                                    ident: `is_two`,
                                    token_idx: TokenIdx(
                                        114,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 14,
                    use_expr_idx: 11,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_three`,
                            token_idx: TokenIdx(
                                48,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::three`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::three::is_three`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 29,
                                ident_token: IdentToken {
                                    ident: `is_three`,
                                    token_idx: TokenIdx(
                                        27,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 15,
                    use_expr_idx: 14,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_five`,
                            token_idx: TokenIdx(
                                54,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::five`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::five::is_five`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 2,
                                ident_token: IdentToken {
                                    ident: `is_five`,
                                    token_idx: TokenIdx(
                                        9,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 16,
                    use_expr_idx: 17,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_seven`,
                            token_idx: TokenIdx(
                                60,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::seven`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 45,
                                ident_token: IdentToken {
                                    ident: `is_seven`,
                                    token_idx: TokenIdx(
                                        166,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 17,
                    use_expr_idx: 20,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_eight`,
                            token_idx: TokenIdx(
                                66,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::eight`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 19,
                                ident_token: IdentToken {
                                    ident: `is_eight`,
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 23,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    variant: UseExprRuleVariant::Leaf {
                        ident_token: IdentToken {
                            ident: `is_nine`,
                            token_idx: TokenIdx(
                                72,
                            ),
                        },
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier::digits::nine`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::ModuleItem(
                            ModuleItemSymbol {
                                path: ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                ),
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier::digits`,
                                ),
                                ast_idx: 38,
                                ident_token: IdentToken {
                                    ident: `is_nine`,
                                    token_idx: TokenIdx(
                                        37,
                                    ),
                                },
                            },
                        ),
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
                    ast_idx: 19,
                    use_expr_idx: 26,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                    progress: 28,
                },
            ],
        ),
        errors: [],
    },
)