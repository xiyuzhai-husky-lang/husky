Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::digits`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "zero",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::zero`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 0,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "one",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::one`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 1,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "six",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::six`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 2,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "three",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::three`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 3,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "four",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::four`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 4,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "five",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::five`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 5,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "six",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::six`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 6,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "seven",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::seven`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 7,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "eight",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::eight`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 8,
                        },
                    ),
                },
                NativeEntitySymbolEntry {
                    ident: Identifier(
                        "nine",
                    ),
                    accessibility: Accessibility::PublicUnder(
                        `mnist_classifier::digits`,
                    ),
                    symbol: NativeEntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits::nine`,
                            accessibility: Accessibility::PublicUnder(
                                `mnist_classifier::digits`,
                            ),
                            ast_idx: 9,
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 10,
                    use_expr_idx: 1,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::Public,
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 119,
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
                    ast_idx: 11,
                    use_expr_idx: 3,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
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
                    ast_idx: 12,
                    use_expr_idx: 5,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 124,
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
                    ast_idx: 13,
                    use_expr_idx: 7,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 123,
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
                    ast_idx: 14,
                    use_expr_idx: 9,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 122,
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
                    ast_idx: 15,
                    use_expr_idx: 11,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 116,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 16,
                    use_expr_idx: 13,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
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
                                    46,
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
                    ast_idx: 17,
                    use_expr_idx: 15,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 120,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 17,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 115,
                                        },
                                    ),
                                ),
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
                    state: UseExprRuleState::Unresolved,
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 19,
                    accessibility: AccessibilityProgress::Done {
                        accessibility: Accessibility::PublicUnder(
                            `mnist_classifier::digits`,
                        ),
                    },
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: Identifier(
                            IdentifierToken {
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 118,
                                        },
                                    ),
                                ),
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
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 144,
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
                                        value: 119,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                22,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                23,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 0,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 145,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            28,
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
                                26,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                27,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 2,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 146,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            32,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 124,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                31,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 4,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 152,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            36,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 123,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                34,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                35,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 6,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 149,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            40,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 122,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                38,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                39,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 8,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 151,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            44,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 116,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                42,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                43,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 10,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 145,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            48,
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
                                46,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                47,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 12,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 147,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            52,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 120,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                50,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                51,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 14,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 148,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            56,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 115,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                54,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                55,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 16,
                        },
                    ),
                },
                Leaf {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 150,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            60,
                        ),
                    },
                },
                Parent {
                    parent_name_token: Identifier(
                        IdentifierToken {
                            ident: Identifier(
                                Word(
                                    Id {
                                        value: 118,
                                    },
                                ),
                            ),
                            token_idx: TokenIdx(
                                58,
                            ),
                        },
                    ),
                    scope_resolution_token: Ok(
                        ScopeResolutionToken {
                            token_idx: TokenIdx(
                                59,
                            ),
                        },
                    ),
                    children: Ok(
                        Single {
                            child: 18,
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