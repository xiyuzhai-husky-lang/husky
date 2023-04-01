Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist_classifier`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 61,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 62,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 63,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 64,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 65,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 66,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 67,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 68,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 69,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 81,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 86,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 87,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 234,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 280,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 281,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 282,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `mnist`,
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
                                original_symbol: PackageDependency {
                                    entity_path: Module(
                                        ModulePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 20,
                            use_expr_idx: 3,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `mnist`,
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: PackageDependency {
                                    entity_path: Module(
                                        ModulePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 5,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 22,
                            use_expr_idx: 7,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `digits`,
                                        token_idx: TokenIdx(
                                            34,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 23,
                            use_expr_idx: 9,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    8..9,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 24,
                            use_expr_idx: 11,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    10..11,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 25,
                            use_expr_idx: 13,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 26,
                            use_expr_idx: 18,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    17..18,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 27,
                            use_expr_idx: 20,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `mnist`,
                                        token_idx: TokenIdx(
                                            61,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: PackageDependency {
                                    entity_path: Module(
                                        ModulePath(
                                            Id {
                                                value: 62,
                                            },
                                        ),
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 0,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `BinaryImage28`,
                                    token_idx: TokenIdx(
                                        24,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 20,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `MnistLabel`,
                                    token_idx: TokenIdx(
                                        28,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: PackageDependency {
                                    entity_path: ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 79,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 24,
                            use_expr_idx: 10,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `find_raw_contours`,
                                    token_idx: TokenIdx(
                                        45,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::raw_contour`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 129,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            },
                            ast_idx: 21,
                            use_expr_idx: 4,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 28,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            },
                            ast_idx: 22,
                            use_expr_idx: 6,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 41,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 45,
                                    },
                                ),
                            },
                            ast_idx: 23,
                            use_expr_idx: 8,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 24,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            },
                            ast_idx: 25,
                            use_expr_idx: 12,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Outside,
                                path: ModulePath(
                                    Id {
                                        value: 62,
                                    },
                                ),
                            },
                            ast_idx: 27,
                            use_expr_idx: 19,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 1,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `BinaryImage28`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `find_raw_contours`,
                                token_idx: TokenIdx(
                                    45,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::connected_component`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `ConnectedComponentDistribution`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EffHoles`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `hole_tmpl`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 115,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponent`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 116,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `horizontal_extend`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 117,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_connected_components`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 94,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 95,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 96,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 97,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 98,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 99,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 100,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 101,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 102,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 103,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 104,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 105,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 106,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 107,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 108,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 109,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 110,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 111,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 112,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 283,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 611,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 612,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 613,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 614,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 124,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    77,
                                ),
                            },
                            ty_expr: 0,
                            body: ArenaIdxRange(
                                67..79,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 118,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 119,
                            use_expr_idx: 4,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 118,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 35,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            },
                            ast_idx: 119,
                            use_expr_idx: 3,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            },
                            ast_idx: 118,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                            progress: 43,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::raw_contour`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 119,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Direction`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 120,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_pixel_pair`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_pixel_to_the_left`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 122,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_pixel_to_the_right`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 123,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_inward_direction`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_angle_change`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 125,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_outward_direction`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 126,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `StreakCache`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 127,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `get_concave_middle_point`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 129,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 115,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 116,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 117,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 119,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 120,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 122,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 123,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 125,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 126,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 127,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 128,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 129,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 131,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 312,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 313,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 314,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 315,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 316,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 317,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponentDistribution`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 318,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EffHoles`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 319,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 321,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 323,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 355,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 356,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 357,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 628,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 200,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    38,
                                ),
                            },
                            ty_expr: 1,
                            body: ArenaIdxRange(
                                27..34,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 195,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 196,
                            use_expr_idx: 5,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
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
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 197,
                            use_expr_idx: 8,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
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
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 198,
                            use_expr_idx: 10,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 195,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 36,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 196,
                            use_expr_idx: 4,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `connected_component`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 34,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 197,
                            use_expr_idx: 7,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            },
                            ast_idx: 198,
                            use_expr_idx: 9,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            },
                            ast_idx: 195,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            },
                            ast_idx: 196,
                            use_expr_idx: 3,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            progress: 30,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            },
                            ast_idx: 197,
                            use_expr_idx: 6,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::geom2d`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 130,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 131,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 134,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 79,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                            ty_expr: 2,
                            body: ArenaIdxRange(
                                5..10,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 82,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    168,
                                ),
                            },
                            ty_expr: 3,
                            body: ArenaIdxRange(
                                41..49,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `mnist_classifier::geom2d`,
                                disambiguator: 0,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    499,
                                ),
                            },
                            ast_idx: 84,
                            body: ArenaIdxRange(
                                56..58,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `CloseRange`,
                                            token_idx: TokenIdx(
                                                500,
                                            ),
                                        },
                                    ),
                                ),
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 86,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    609,
                                ),
                            },
                            ty_expr: 4,
                            body: ArenaIdxRange(
                                64..70,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 88,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    743,
                                ),
                            },
                            ty_expr: 5,
                            body: ArenaIdxRange(
                                74..78,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 136,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 137,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 139,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 141,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::Pub,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::Pub,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::Pub,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 358,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 378,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 379,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 380,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 381,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 382,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 383,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 170,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    81,
                                ),
                            },
                            ty_expr: 6,
                            body: ArenaIdxRange(
                                3..7,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 172,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    175,
                                ),
                            },
                            ty_expr: 7,
                            body: ArenaIdxRange(
                                21..26,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 163,
                            use_expr_idx: 1,
                            visibility: Visibility::Pub,
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            10,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 41,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 164,
                            use_expr_idx: 3,
                            visibility: Visibility::Pub,
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `convex_component`,
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 165,
                            use_expr_idx: 5,
                            visibility: Visibility::Pub,
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment`,
                                        token_idx: TokenIdx(
                                            20,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 44,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 166,
                            use_expr_idx: 8,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            24,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 167,
                            use_expr_idx: 11,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    10..11,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 168,
                            use_expr_idx: 13,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment`,
                                        token_idx: TokenIdx(
                                            36,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 44,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 166,
                            use_expr_idx: 7,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            26,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 36,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 167,
                            use_expr_idx: 10,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            32,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 167,
                            use_expr_idx: 9,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `RawContour`,
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::raw_contour`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 119,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            },
                            ast_idx: 163,
                            use_expr_idx: 0,
                            visibility: Pub,
                            progress: 40,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 49,
                                    },
                                ),
                            },
                            ast_idx: 164,
                            use_expr_idx: 2,
                            visibility: Pub,
                            progress: 22,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            },
                            ast_idx: 165,
                            use_expr_idx: 4,
                            visibility: Pub,
                            progress: 7,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            },
                            ast_idx: 168,
                            use_expr_idx: 12,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                            progress: 7,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            },
                            ast_idx: 166,
                            use_expr_idx: 6,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 137,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 138,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 139,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 140,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 141,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 145,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 146,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 147,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 148,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 149,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 150,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 151,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 153,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 154,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 155,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 384,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 385,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 386,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 387,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 390,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 391,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 392,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 393,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 394,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 397,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 398,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 399,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 400,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 401,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 402,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 403,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 404,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 689,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_convex`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 700,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 75,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                            ty_expr: 8,
                            body: ArenaIdxRange(
                                39..53,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 69,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 70,
                            use_expr_idx: 6,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    5..6,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 71,
                            use_expr_idx: 10,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 72,
                            use_expr_idx: 13,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 73,
                            use_expr_idx: 15,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            29,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 69,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 70,
                            use_expr_idx: 5,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 71,
                            use_expr_idx: 9,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            17,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    8..9,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 72,
                            use_expr_idx: 12,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    11..12,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 36,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 70,
                            use_expr_idx: 4,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment`,
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 44,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 71,
                            use_expr_idx: 8,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `convexity`,
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 43,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            },
                            ast_idx: 73,
                            use_expr_idx: 14,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            },
                            ast_idx: 69,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            },
                            ast_idx: 72,
                            use_expr_idx: 11,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            },
                            ast_idx: 70,
                            use_expr_idx: 3,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 7,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            },
                            ast_idx: 71,
                            use_expr_idx: 7,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 22,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `ConvexComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 145,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 411,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 412,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 413,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 414,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 415,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 416,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 418,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 419,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 421,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 423,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 424,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 425,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 426,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 428,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 430,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 431,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 4,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                            ty_expr: 9,
                            body: ArenaIdxRange(
                                0..2,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 2,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 2,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            },
                            ast_idx: 2,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 49,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::convexity`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `is_convex`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 146,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `concave_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 432,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 433,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 435,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 436,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 439,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 441,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 442,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 444,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 445,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 447,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 449,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 451,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 20,
                            use_expr_idx: 5,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
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
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 8,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
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
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 20,
                            use_expr_idx: 4,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 7,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 36,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            },
                            ast_idx: 19,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            },
                            ast_idx: 20,
                            use_expr_idx: 3,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            progress: 43,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            },
                            ast_idx: 21,
                            use_expr_idx: 6,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 147,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 501,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 502,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 503,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 504,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 505,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 506,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                disambiguator: 0,
                            },
                            ast_idx: 17,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                            ty_expr: 10,
                            body: ArenaIdxRange(
                                13..15,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 15,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 15,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `geom2d`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 36,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            },
                            ast_idx: 15,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::fermi`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `FermiMatchResult`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 148,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 149,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 156,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 157,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 158,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 159,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 160,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 161,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 162,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 163,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 164,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 165,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 166,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 167,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 168,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 169,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 170,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 171,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 173,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 174,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 722,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 723,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 724,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 725,
                                    },
                                ),
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
                                    23,
                                ),
                            },
                            ty_expr: 11,
                            body: ArenaIdxRange(
                                12..15,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
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
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            },
                            ast_idx: 21,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 45,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `zero`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 45,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `three`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `four`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 49,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `five`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `seven`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `eight`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 52,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `two`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::Pub,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 176,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_zero`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 177,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_two`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 178,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_three`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 179,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_five`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 180,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_seven`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 181,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 182,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_nine`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 183,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 184,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 185,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 186,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 187,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 188,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 189,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 190,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 191,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 193,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 194,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 196,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 198,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 199,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 200,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 202,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 727,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 728,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 729,
                                    },
                                ),
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
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 46,
                                        },
                                    ),
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
                                            26,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 12,
                            use_expr_idx: 5,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `zero`,
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 13,
                            use_expr_idx: 7,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `two`,
                                        token_idx: TokenIdx(
                                            34,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 54,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 14,
                            use_expr_idx: 9,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `three`,
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    8..9,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 48,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 15,
                            use_expr_idx: 11,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `five`,
                                        token_idx: TokenIdx(
                                            42,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    10..11,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 50,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 16,
                            use_expr_idx: 13,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `seven`,
                                        token_idx: TokenIdx(
                                            46,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 51,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 17,
                            use_expr_idx: 15,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `eight`,
                                        token_idx: TokenIdx(
                                            50,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 52,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 17,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `nine`,
                                        token_idx: TokenIdx(
                                            54,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    16..17,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 53,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 19,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            58,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    18..19,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
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
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 154,
                                        },
                                    ),
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
                                        28,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::six`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 160,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 12,
                            use_expr_idx: 4,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_zero`,
                                    token_idx: TokenIdx(
                                        32,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::zero`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 152,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 13,
                            use_expr_idx: 6,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_two`,
                                    token_idx: TokenIdx(
                                        36,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::two`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 194,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 14,
                            use_expr_idx: 8,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_three`,
                                    token_idx: TokenIdx(
                                        40,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::three`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 164,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 15,
                            use_expr_idx: 10,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_five`,
                                    token_idx: TokenIdx(
                                        44,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::five`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 175,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 16,
                            use_expr_idx: 12,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_seven`,
                                    token_idx: TokenIdx(
                                        48,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::seven`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 181,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 17,
                            use_expr_idx: 14,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_eight`,
                                    token_idx: TokenIdx(
                                        52,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::eight`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 183,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 16,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_nine`,
                                    token_idx: TokenIdx(
                                        56,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::digits::nine`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 187,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            },
                            ast_idx: 19,
                            use_expr_idx: 18,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::zero`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `open_one_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 150,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `almost_closed`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 151,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_zero`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::one`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `one_fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 153,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 154,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `upmost`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 155,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `downmost`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 156,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `hat`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 157,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::six`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `six_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 158,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six_match_refined1`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 159,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 160,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `upmost`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 161,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `bottom1`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 162,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::three`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `three_fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 163,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_three`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 164,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `uparc`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 165,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `downarc`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 166,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `back`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 167,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::four`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `left_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 168,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `left_coordinate_max`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 169,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `components_max_downwards`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 170,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `components_max_heights`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 171,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_four`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 172,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `displacement_downwards`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 173,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `cc_box_heights`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 174,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::five`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `is_five`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::seven`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `simple_seven_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 176,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `simple_leftdown_pattern`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 177,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `special_seven_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 178,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `leftupcc_pattern`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 179,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `leftdowncc_pattern`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 180,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_seven`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 181,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::eight`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `upper_mouth_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 182,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 183,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `big_mouth`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 184,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::nine`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `nine_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 185,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine_match_refine`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 186,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_nine`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 187,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `downmost`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 188,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `big_cc`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 189,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::two`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `two_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 190,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `left_cc_pattern`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 191,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `right_cc_pattern`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 192,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `down_cc_pattern`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 193,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_two`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 194,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::major`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 196,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 198,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 199,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 200,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::Pub,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 201,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 203,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 204,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 205,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 206,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 207,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 208,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 209,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MnistLabel`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 210,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 211,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 219,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 220,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 221,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponentDistribution`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 507,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EffHoles`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 508,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 510,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_connected_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 512,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 533,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 586,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 587,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 588,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 730,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 2,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
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
                                    1..2,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 5,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
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
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 20,
                            use_expr_idx: 8,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
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
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 12,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    11..12,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 22,
                            use_expr_idx: 14,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            27,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    13..14,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Resolved {
                                original_symbol: CrateRoot {
                                    root_module: ModulePath(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                },
                            },
                        },
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 1,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `connected_component`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 34,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 4,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 35,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 20,
                            use_expr_idx: 7,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 11,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            21,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    10..11,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 37,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 10,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier::major`,
                            ),
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 41,
                                        },
                                    ),
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
                                path: ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            },
                            ast_idx: 22,
                            use_expr_idx: 13,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            },
                            ast_idx: 18,
                            use_expr_idx: 0,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 30,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            },
                            ast_idx: 19,
                            use_expr_idx: 3,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 43,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            },
                            ast_idx: 20,
                            use_expr_idx: 6,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            },
                            ast_idx: 21,
                            use_expr_idx: 9,
                            visibility: PubUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 40,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                    ),
                ],
            },
        ],
        principal_entity_path_expr_arena: Arena {
            data: [
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `ConnectedComponent`,
                        token_idx: TokenIdx(
                            78,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `RawContour`,
                        token_idx: TokenIdx(
                            39,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Point2d`,
                        token_idx: TokenIdx(
                            14,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Vector2d`,
                        token_idx: TokenIdx(
                            169,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `BoundingBox`,
                        token_idx: TokenIdx(
                            610,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `RelativeBoundingBox`,
                        token_idx: TokenIdx(
                            744,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `LineSegmentStroke`,
                        token_idx: TokenIdx(
                            82,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `LineSegmentSketch`,
                        token_idx: TokenIdx(
                            176,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `ConcaveComponent`,
                        token_idx: TokenIdx(
                            50,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `ConvexComponent`,
                        token_idx: TokenIdx(
                            23,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `LineSegment`,
                        token_idx: TokenIdx(
                            20,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `FermiMatchResult`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        ),
                    ),
                },
            ],
        },
    },
)