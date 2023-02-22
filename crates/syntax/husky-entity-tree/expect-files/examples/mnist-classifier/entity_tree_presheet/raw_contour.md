Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::raw_contour`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "RawContour",
                    ),
                    accessibility: Accessibility::Public,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            accessibility: Accessibility::Public,
                            ast_idx: 199,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "Direction",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 201,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_pixel_pair",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 206,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_pixel_to_the_left",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 207,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_pixel_to_the_right",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 208,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_inward_direction",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 209,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_angle_change",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_angle_change`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 210,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_outward_direction",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 211,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "StreakCache",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 212,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "get_concave_middle_point",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 213,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "find_raw_contours",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::raw_contour`,
                    ),
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Form(
                                FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Function`),
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::raw_contour`,
                            ),
                            ast_idx: 214,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 195,
                    use_expr_idx: 2,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 196,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 197,
                    use_expr_idx: 8,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 198,
                    use_expr_idx: 11,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::raw_contour`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 121,
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
                    state: UseExprRuleState::Unresolved,
                },
            ],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 113,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 0,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                1,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 1,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            11,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 100,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 3,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 4,
                        },
                    ),
                },
                All {
                    star_token: StarToken {
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 114,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                15,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 6,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Crate(
                        CrateToken {
                            token_idx: TokenIdx(
                                13,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                14,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 7,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 123,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            22,
                        ),
                    },
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 430,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 121,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                    ),
                    children: Ok(
                        Multiple {
                            lcurl_token: LeftCurlyBraceToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                            children: ArenaIdxRange(
                                9..11,
                            ),
                            comma_tokens: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        23,
                                    ),
                                },
                            ],
                            rcurl_token: Ok(
                                RightCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        25,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ],
        },
        mod_path_arena: Arena {
            data: [],
        },
        errors: [],
    },
)