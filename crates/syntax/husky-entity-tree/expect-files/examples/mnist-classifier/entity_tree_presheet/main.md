Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "connected_component",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::connected_component`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 12,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "raw_contour",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::raw_contour`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 13,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "geom2d",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::geom2d`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 14,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "line_segment_sketch",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 15,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "fermi",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::fermi`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 16,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "digits",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 17,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Ident(
                        "major",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::major`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 1,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 144,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 20,
                    use_expr_idx: 3,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 144,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 142,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 22,
                    use_expr_idx: 7,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 124,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 23,
                    use_expr_idx: 9,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 135,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 24,
                    use_expr_idx: 11,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 143,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 25,
                    use_expr_idx: 13,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 137,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 26,
                    use_expr_idx: 18,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Ident(
                            IdentToken {
                                ident: Ident(
                                    Word(
                                        Id {
                                            value: 149,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
            ],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 146,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                22,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                23,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 0,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            28,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 144,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                26,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                27,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 2,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            32,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 142,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                31,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 4,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            36,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                34,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                35,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 6,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            40,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 135,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                38,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                39,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 8,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 148,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            45,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 143,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                43,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                44,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 10,
                        },
                    ),
                },
                All {
                    star_token: StarToken(
                        TokenIdx(
                            49,
                        ),
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 137,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                47,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                48,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 12,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentToken {
                        ident: Ident(
                            Word(
                                Id {
                                    value: 153,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            59,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 152,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                57,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                58,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 14,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 151,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                55,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                56,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 15,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 150,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                53,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                54,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 16,
                        },
                    ),
                },
                Parent {
                    parent_name_token: Ident(
                        IdentToken {
                            ident: Ident(
                                Word(
                                    Id {
                                        value: 149,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                51,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken(
                            TokenIdx(
                                52,
                            ),
                        ),
                    ),
                    children: Ok(
                        Single {
                            child: 17,
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