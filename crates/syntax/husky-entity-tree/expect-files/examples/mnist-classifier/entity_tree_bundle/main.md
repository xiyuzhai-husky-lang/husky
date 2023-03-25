Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `mnist_classifier`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            ident: `find_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 60,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 76,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 82,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
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
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 228,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 229,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 230,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 24,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 21,
                            use_expr_idx: 4,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 46,
                                },
                            ),
                            ast_idx: 22,
                            use_expr_idx: 6,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 18,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 23,
                            use_expr_idx: 8,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                            ast_idx: 25,
                            use_expr_idx: 12,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 62,
                                },
                            ),
                            ast_idx: 27,
                            use_expr_idx: 19,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            progress: 0,
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
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `MnistLabel`,
                                token_idx: TokenIdx(
                                    28,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 88,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 89,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 90,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 91,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 92,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 93,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `find_raw_contours`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 231,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 832,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 833,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 834,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::connected_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 835,
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
                            ast_idx: 125,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    85,
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
                            use_expr_idx: 3,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::connected_component`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `cv`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 119,
                            use_expr_idx: 6,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::connected_component`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            9,
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
                            ast_idx: 120,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::connected_component`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            15,
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
                            ast_idx: 119,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::connected_component`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            11,
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
                            parent: ModulePath(
                                Id {
                                    value: 40,
                                },
                            ),
                            ast_idx: 120,
                            use_expr_idx: 7,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                            progress: 22,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 42,
                                },
                            ),
                            ast_idx: 119,
                            use_expr_idx: 4,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                            progress: 26,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `cv`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::raw_contour`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            ident: `Point2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 242,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 243,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 244,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 245,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 246,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 247,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponentDistribution`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 248,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EffHoles`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 249,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 251,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 253,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 277,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 278,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 284,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 285,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 286,
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
                                    42,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
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
                            use_expr_idx: 11,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `mnist`,
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
                            ast_idx: 195,
                            use_expr_idx: 1,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
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
                            ast_idx: 198,
                            use_expr_idx: 9,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `BinaryImage28`,
                                    token_idx: TokenIdx(
                                        22,
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
                            ast_idx: 198,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `BinaryGrid28`,
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
                    ],
                ),
                use_all_rules: UseAllRules(
                    [
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 43,
                                },
                            ),
                            ast_idx: 195,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 41,
                                },
                            ),
                            ast_idx: 196,
                            use_expr_idx: 3,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                            progress: 29,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                            ast_idx: 197,
                            use_expr_idx: 6,
                            accessibility: PublicUnder(
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
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `BinaryImage28`,
                                token_idx: TokenIdx(
                                    22,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `BinaryGrid28`,
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::geom2d`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 106,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 107,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 109,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 287,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 306,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 307,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 308,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 309,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 310,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 311,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 163,
                            use_expr_idx: 0,
                            accessibility: Public,
                            progress: 39,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 49,
                                },
                            ),
                            ast_idx: 164,
                            use_expr_idx: 2,
                            accessibility: Public,
                            progress: 22,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 51,
                                },
                            ),
                            ast_idx: 165,
                            use_expr_idx: 4,
                            accessibility: Public,
                            progress: 7,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 51,
                                },
                            ),
                            ast_idx: 168,
                            use_expr_idx: 12,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                            progress: 7,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 43,
                                },
                            ),
                            ast_idx: 166,
                            use_expr_idx: 6,
                            accessibility: PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `geom2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `fermi`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `digits`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `major`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `find_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `concave_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `convex_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `convexity`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `line_segment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `go_right`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `go_left`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `extend_end`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 320,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                            ident: `find_line_segments`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 322,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 325,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 326,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 327,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 328,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 329,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 330,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 331,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 332,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 908,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_convex`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::concave_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 919,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::concave_component`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 40,
                                },
                            ),
                            ast_idx: 73,
                            use_expr_idx: 14,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 22,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                            ast_idx: 69,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 43,
                                },
                            ),
                            ast_idx: 72,
                            use_expr_idx: 11,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 6,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 51,
                                },
                            ),
                            ast_idx: 70,
                            use_expr_idx: 3,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                            progress: 7,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 50,
                                },
                            ),
                            ast_idx: 71,
                            use_expr_idx: 7,
                            accessibility: PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 339,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 340,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 341,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 342,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 343,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 344,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 345,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 346,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 347,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 348,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 349,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 350,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 351,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 352,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 353,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 354,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
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
                            ident: `Vector2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
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
                            ident: `ClosedRange`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
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
                            ident: `BoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
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
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convex_component`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 359,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convex_component`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convex_component`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                            ast_idx: 2,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 360,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convex_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 361,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `convexity`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 362,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `line_segment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 363,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 364,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 365,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_right`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 366,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `go_left`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 367,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_end`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 368,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `extend_start`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 369,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_line_segments`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 370,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 371,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 372,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 373,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 374,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Point2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 375,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 376,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Vector2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 377,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ClosedRange`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
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
                            ident: `BoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
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
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::convexity`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 380,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::convexity`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                            ast_idx: 19,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 42,
                                },
                            ),
                            ast_idx: 20,
                            use_expr_idx: 3,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                            progress: 26,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 43,
                                },
                            ),
                            ast_idx: 21,
                            use_expr_idx: 6,
                            accessibility: PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
                            ident: `RelativePoint2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
                            ident: `Vector2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
                            ident: `ClosedRange`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
                            ident: `BoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
                            ident: `RelativeBoundingBox`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::line_segment_sketch::line_segment`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 418,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::line_segment`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::line_segment_sketch::line_segment`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 43,
                                },
                            ),
                            ast_idx: 15,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `geom2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 132,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 133,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `digits`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 134,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 136,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
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
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 941,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 942,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 943,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::fermi`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 944,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::fermi`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 40,
                                },
                            ),
                            ast_idx: 21,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 45,
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
                module_path: `mnist_classifier::digits`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `zero`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 147,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
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
                            ident: `is_zero`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
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
                            ident: `is_two`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
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
                            ident: `is_three`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
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
                            ident: `is_seven`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
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
                            ident: `is_eight`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
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
                            ident: `is_nine`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 155,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `six`,
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
                                            value: 47,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 17,
                            use_expr_idx: 15,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `seven`,
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
                                            value: 51,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 17,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `eight`,
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
                                            value: 52,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 19,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `nine`,
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
                            ast_idx: 10,
                            use_expr_idx: 0,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                                            value: 193,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 14,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
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
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 16,
                            use_expr_idx: 12,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_six`,
                                    token_idx: TokenIdx(
                                        48,
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
                            ast_idx: 17,
                            use_expr_idx: 14,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_seven`,
                                    token_idx: TokenIdx(
                                        52,
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
                                            value: 180,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 16,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_eight`,
                                    token_idx: TokenIdx(
                                        56,
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
                                            value: 182,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 18,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `is_nine`,
                                    token_idx: TokenIdx(
                                        60,
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
                                            value: 186,
                                        },
                                    ),
                                ),
                            },
                        },
                    ],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_six`,
                                token_idx: TokenIdx(
                                    28,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_zero`,
                                token_idx: TokenIdx(
                                    32,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_two`,
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_three`,
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `is_five`,
                                token_idx: TokenIdx(
                                    44,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_six`,
                                token_idx: TokenIdx(
                                    48,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_seven`,
                                token_idx: TokenIdx(
                                    52,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_eight`,
                                token_idx: TokenIdx(
                                    56,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `is_nine`,
                                token_idx: TokenIdx(
                                    60,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::zero`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `open_one_match`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
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
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 953,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::zero`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 954,
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
                            ast_idx: 25,
                            use_expr_idx: 3,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `cv`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 26,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 27,
                            use_expr_idx: 13,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 28,
                            use_expr_idx: 18,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            29,
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
                            ast_idx: 29,
                            use_expr_idx: 21,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    20..21,
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
                            ast_idx: 30,
                            use_expr_idx: 24,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    23..24,
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
                            ast_idx: 31,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            ast_idx: 32,
                            use_expr_idx: 31,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    30..31,
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
                            ast_idx: 29,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 30,
                            use_expr_idx: 23,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    22..23,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 31,
                            use_expr_idx: 27,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            53,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    26..27,
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
                            ast_idx: 32,
                            use_expr_idx: 30,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `one`,
                                        token_idx: TokenIdx(
                                            61,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    29..30,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 31,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::zero`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 29,
                            use_expr_idx: 19,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 52,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 30,
                            use_expr_idx: 22,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 52,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 31,
                            use_expr_idx: 25,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 52,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `cv`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::one`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `one_fermi_match`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::Public,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 456,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 458,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 460,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 461,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 462,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 475,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1000,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::one`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1001,
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
                            ast_idx: 61,
                            use_expr_idx: 3,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `cv`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 62,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 63,
                            use_expr_idx: 13,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 64,
                            use_expr_idx: 18,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            29,
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
                            ast_idx: 65,
                            use_expr_idx: 21,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    20..21,
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
                            ast_idx: 66,
                            use_expr_idx: 24,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    23..24,
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
                            ast_idx: 67,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            ast_idx: 65,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 66,
                            use_expr_idx: 23,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    22..23,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 67,
                            use_expr_idx: 27,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            53,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    26..27,
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
                            ast_idx: 67,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::one`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 65,
                            use_expr_idx: 19,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 66,
                            use_expr_idx: 22,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 67,
                            use_expr_idx: 25,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `cv`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::six`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `six_match`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 493,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 494,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 495,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 496,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 497,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 498,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 499,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
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
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1047,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::six`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1048,
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
                            ast_idx: 49,
                            use_expr_idx: 3,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `cv`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    2..3,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 50,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 51,
                            use_expr_idx: 13,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 52,
                            use_expr_idx: 18,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            29,
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
                            ast_idx: 53,
                            use_expr_idx: 21,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    20..21,
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
                            ast_idx: 54,
                            use_expr_idx: 24,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    23..24,
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
                            ast_idx: 55,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            ast_idx: 56,
                            use_expr_idx: 31,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    30..31,
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
                            ast_idx: 53,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 54,
                            use_expr_idx: 23,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    22..23,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 55,
                            use_expr_idx: 27,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            53,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    26..27,
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
                            ast_idx: 56,
                            use_expr_idx: 30,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `one`,
                                        token_idx: TokenIdx(
                                            61,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    29..30,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 55,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::six`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 53,
                            use_expr_idx: 19,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 54,
                            use_expr_idx: 22,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 55,
                            use_expr_idx: 25,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `cv`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    61,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::three`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `three_fermi_match`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 530,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 531,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 532,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 534,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 535,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 536,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 549,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1094,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::three`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1095,
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
                            ast_idx: 27,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 28,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 29,
                            use_expr_idx: 15,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 30,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 31,
                            use_expr_idx: 23,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    22..23,
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
                            ast_idx: 32,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            49,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            ast_idx: 33,
                            use_expr_idx: 30,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    29..30,
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
                            ast_idx: 31,
                            use_expr_idx: 22,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    21..22,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 32,
                            use_expr_idx: 25,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    24..25,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 33,
                            use_expr_idx: 29,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            57,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    28..29,
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
                            ast_idx: 33,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::three`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 31,
                            use_expr_idx: 21,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 32,
                            use_expr_idx: 24,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 33,
                            use_expr_idx: 27,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::four`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `left_components`,
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                            visibility: Visibility::PublicUnder(
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
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 567,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 568,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 569,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 570,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 571,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 572,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 573,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
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
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1141,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::four`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1142,
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
                            ast_idx: 37,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 38,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 39,
                            use_expr_idx: 15,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 40,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 41,
                            use_expr_idx: 23,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    22..23,
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
                            ast_idx: 42,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            49,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            ast_idx: 43,
                            use_expr_idx: 30,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    29..30,
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
                            ast_idx: 41,
                            use_expr_idx: 22,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    21..22,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 42,
                            use_expr_idx: 25,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    24..25,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 43,
                            use_expr_idx: 29,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            57,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    28..29,
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
                            ast_idx: 43,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::four`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 41,
                            use_expr_idx: 21,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 42,
                            use_expr_idx: 24,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 43,
                            use_expr_idx: 27,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::five`,
                symbols: EntitySymbolTable(
                    [],
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `simple_leftdown_pattern`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `special_seven_match`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `leftupcc_pattern`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `leftdowncc_pattern`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `is_seven`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 604,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 605,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 606,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 608,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 609,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 610,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 625,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `zero`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 643,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 644,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `six`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 645,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `three`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 646,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `four`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 647,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `five`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 648,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `seven`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 649,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `eight`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 650,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 651,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `two`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 652,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 653,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_six`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 654,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_zero`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 655,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_two`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 656,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_three`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 657,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 659,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_nine`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 660,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1188,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::seven`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1189,
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
                            ast_idx: 39,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 40,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
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
                            ast_idx: 41,
                            use_expr_idx: 11,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            19,
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
                            ast_idx: 42,
                            use_expr_idx: 16,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    15..16,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 43,
                            use_expr_idx: 19,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            35,
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
                            ast_idx: 44,
                            use_expr_idx: 22,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    21..22,
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
                            ast_idx: 45,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            ast_idx: 46,
                            use_expr_idx: 29,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    28..29,
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
                            ast_idx: 40,
                            use_expr_idx: 7,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 41,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            21,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 43,
                            use_expr_idx: 18,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    17..18,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 44,
                            use_expr_idx: 21,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    20..21,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 45,
                            use_expr_idx: 25,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            49,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    24..25,
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
                            ast_idx: 46,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `digits`,
                                        token_idx: TokenIdx(
                                            57,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                                            value: 39,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 40,
                            use_expr_idx: 6,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `major_concave_components`,
                                    token_idx: TokenIdx(
                                        17,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::major`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 41,
                            use_expr_idx: 9,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `major_connected_component`,
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::major`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 195,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 45,
                            use_expr_idx: 24,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::seven`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    23..24,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 43,
                            use_expr_idx: 17,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 44,
                            use_expr_idx: 20,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 46,
                                },
                            ),
                            ast_idx: 46,
                            use_expr_idx: 27,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            progress: 18,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 45,
                            use_expr_idx: 23,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    25,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::eight`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `upper_mouth_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 181,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_eight`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `big_mouth`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 661,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 662,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 664,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 682,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1227,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::eight`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1228,
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
                            ast_idx: 17,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 18,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
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
                            use_expr_idx: 11,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            19,
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
                            ast_idx: 20,
                            use_expr_idx: 16,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    15..16,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 19,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            35,
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
                            ast_idx: 22,
                            use_expr_idx: 22,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    21..22,
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
                            ast_idx: 23,
                            use_expr_idx: 25,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    24..25,
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
                            ast_idx: 24,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            53,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            ast_idx: 25,
                            use_expr_idx: 32,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    31..32,
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
                            ast_idx: 26,
                            use_expr_idx: 35,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            67,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    34..35,
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
                            ast_idx: 27,
                            use_expr_idx: 38,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            73,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    37..38,
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
                            use_expr_idx: 7,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            21,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 21,
                            use_expr_idx: 18,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `one`,
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    17..18,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 22,
                            use_expr_idx: 21,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `six`,
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    20..21,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 23,
                            use_expr_idx: 24,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `zero`,
                                        token_idx: TokenIdx(
                                            49,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    23..24,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 24,
                            use_expr_idx: 27,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `seven`,
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    26..27,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 25,
                            use_expr_idx: 31,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            61,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    30..31,
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
                            ast_idx: 26,
                            use_expr_idx: 34,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            69,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    33..34,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 27,
                            use_expr_idx: 37,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_contour`,
                                        token_idx: TokenIdx(
                                            75,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    36..37,
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
                            ast_idx: 18,
                            use_expr_idx: 6,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `major_concave_components`,
                                    token_idx: TokenIdx(
                                        17,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::major`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 200,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 19,
                            use_expr_idx: 9,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `major_connected_component`,
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier::major`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: ModuleItem(
                                    ModuleItemSymbol(
                                        Id {
                                            value: 195,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 25,
                            use_expr_idx: 30,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::eight`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            63,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    29..30,
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
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 26,
                            use_expr_idx: 33,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 42,
                                },
                            ),
                            ast_idx: 27,
                            use_expr_idx: 36,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                            progress: 26,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 25,
                            use_expr_idx: 29,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    25,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `one`,
                                token_idx: TokenIdx(
                                    37,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    43,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    49,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `seven`,
                                token_idx: TokenIdx(
                                    55,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    63,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::nine`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `nine_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 184,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `nine_match_refine`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `is_nine`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `downmost`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
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
                            ident: `big_cc`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 708,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 709,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 710,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 711,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 712,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 713,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 714,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
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
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1274,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::nine`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1275,
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
                            ast_idx: 35,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 36,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 37,
                            use_expr_idx: 15,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 38,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 39,
                            use_expr_idx: 23,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    22..23,
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
                            ast_idx: 40,
                            use_expr_idx: 26,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            49,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    25..26,
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
                            ast_idx: 41,
                            use_expr_idx: 30,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            55,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    29..30,
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
                            ast_idx: 42,
                            use_expr_idx: 33,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            63,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    32..33,
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
                            ast_idx: 43,
                            use_expr_idx: 36,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            69,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    35..36,
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
                            ast_idx: 39,
                            use_expr_idx: 22,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `major`,
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    21..22,
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
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 40,
                            use_expr_idx: 25,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fermi`,
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    24..25,
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
                                            value: 38,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 41,
                            use_expr_idx: 29,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            57,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    28..29,
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
                            ast_idx: 42,
                            use_expr_idx: 32,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `zero`,
                                        token_idx: TokenIdx(
                                            65,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    31..32,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 43,
                            use_expr_idx: 35,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `six`,
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    34..35,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `mnist_classifier`,
                                ),
                            ),
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 41,
                            use_expr_idx: 28,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::nine`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `concave_component`,
                                        token_idx: TokenIdx(
                                            59,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    27..28,
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
                            parent: ModulePath(
                                Id {
                                    value: 47,
                                },
                            ),
                            ast_idx: 39,
                            use_expr_idx: 21,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                            ),
                            progress: 27,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 45,
                                },
                            ),
                            ast_idx: 40,
                            use_expr_idx: 24,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                            ),
                            progress: 23,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 41,
                            use_expr_idx: 27,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 60,
                                    },
                                ),
                            ),
                            progress: 39,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `zero`,
                                token_idx: TokenIdx(
                                    65,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `six`,
                                token_idx: TokenIdx(
                                    71,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::SymbolExistsButNotAccessible(
                            IdentToken {
                                ident: `concave_component`,
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::digits::two`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `two_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 189,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `left_cc_pattern`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `right_cc_pattern`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `down_cc_pattern`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `is_two`,
                            visibility: Visibility::PublicUnder(
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
                            ident: `connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `find_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `major_connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `major_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `major_raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `major_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 172,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
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
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1313,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1314,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1315,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::digits::two`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1316,
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
                            ast_idx: 49,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::two`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 50,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::two`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            13,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 51,
                            use_expr_idx: 15,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::two`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 52,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::two`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `domains`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
                                ),
                            },
                            parent: None,
                            state: UseExprRuleState::Erroneous,
                        },
                        UseExprRule {
                            ast_idx: 53,
                            use_expr_idx: 22,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::digits::two`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            43,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    21..22,
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
                            parent: ModulePath(
                                Id {
                                    value: 40,
                                },
                            ),
                            ast_idx: 53,
                            use_expr_idx: 21,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 61,
                                    },
                                ),
                            ),
                            progress: 22,
                        },
                    ],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                    ),
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `domains`,
                                token_idx: TokenIdx(
                                    33,
                                ),
                            },
                        ),
                    ),
                ],
            },
            EntityTreeSheet {
                module_path: `mnist_classifier::major`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `connected_components`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 194,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_connected_component`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 195,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ignored_connected_components_row_span_sum_sum`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 196,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contours`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 197,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_raw_contour`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 198,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_line_segment_sketch`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 199,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `major_concave_components`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 200,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `connected_component`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `raw_contour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 175,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `geom2d`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `line_segment_sketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `fermi`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `digits`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `major`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `find_raw_contours`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `fermi_match`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `LineSegmentStroke`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `LineSegmentSketch`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
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
                            ident: `ConnectedComponentDistribution`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 745,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `EffHoles`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 746,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConnectedComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 748,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_connected_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 750,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RawContour`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 770,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ConcaveComponent`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 807,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `find_concave_components`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 808,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LineSegment`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 809,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `is_one`,
                            visibility: Visibility::PublicUnder(
                                `mnist_classifier::major`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 1317,
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `mnist_classifier::major`,
                                ),
                            },
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
                            parent: ModulePath(
                                Id {
                                    value: 40,
                                },
                            ),
                            ast_idx: 22,
                            use_expr_idx: 13,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 22,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 41,
                                },
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 29,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 42,
                                },
                            ),
                            ast_idx: 19,
                            use_expr_idx: 3,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 26,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 44,
                                },
                            ),
                            ast_idx: 20,
                            use_expr_idx: 6,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 21,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 48,
                                },
                            ),
                            ast_idx: 21,
                            use_expr_idx: 9,
                            accessibility: PublicUnder(
                                ModulePath(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                            progress: 39,
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
                            86,
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
                            43,
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