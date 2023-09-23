[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 156,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 156,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            USize(
                                                TermUSizeLiteral(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 7,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 21,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 8,
                                            opr: As,
                                            ropd: 9,
                                        },
                                        FnCall {
                                            function: 3,
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    4,
                                                ),
                                                Regular(
                                                    5,
                                                ),
                                                Regular(
                                                    10,
                                                ),
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Eval {
                                            expr_idx: 11,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 156,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            23,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 195,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 196,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 1,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 2,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 3,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 195,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 196,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function: 4,
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    5,
                                                ),
                                                Regular(
                                                    6,
                                                ),
                                                Regular(
                                                    7,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 195,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 197,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 13,
                                        },
                                        FnCall {
                                            function: 9,
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    10,
                                                ),
                                                Regular(
                                                    11,
                                                ),
                                                Regular(
                                                    14,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 197,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 19,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 196,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function: 16,
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    17,
                                                ),
                                                Regular(
                                                    20,
                                                ),
                                                Regular(
                                                    21,
                                                ),
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                4..5,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Eval {
                                            expr_idx: 15,
                                        },
                                        Eval {
                                            expr_idx: 22,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 3,
                                                stmts: ArenaIdxRange(
                                                    1..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 197,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::partition`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::partition`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            63,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 196,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 1,
                                            opr: As,
                                            ropd: 2,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 195,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 4,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 5,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 196,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            Bool(
                                                true,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 9,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 10,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 13,
                                            opr: As,
                                            ropd: 14,
                                        },
                                        Index {
                                            owner: 12,
                                            items: [
                                                15,
                                            ],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 199,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner: 17,
                                            items: [
                                                18,
                                            ],
                                        },
                                        Binary {
                                            lopd: 16,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 19,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 21,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 22,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 24,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            ropd: 25,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 1,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 27,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 28,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 31,
                                            opr: As,
                                            ropd: 32,
                                        },
                                        Index {
                                            owner: 30,
                                            items: [
                                                33,
                                            ],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 199,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner: 35,
                                            items: [
                                                36,
                                            ],
                                        },
                                        Binary {
                                            lopd: 34,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 37,
                                        },
                                        Binary {
                                            lopd: 29,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 38,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            ISize(
                                                TermISizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 40,
                                            opr: AssignClosed(
                                                Sub,
                                            ),
                                            ropd: 41,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 43,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 44,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 47,
                                            opr: As,
                                            ropd: 48,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 50,
                                            opr: As,
                                            ropd: 51,
                                        },
                                        MethodCall {
                                            self_argument: 46,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 157,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    49,
                                                ),
                                                Regular(
                                                    52,
                                                ),
                                            ],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 193,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 55,
                                            opr: As,
                                            ropd: 56,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 199,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 58,
                                            opr: As,
                                            ropd: 59,
                                        },
                                        MethodCall {
                                            self_argument: 54,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 157,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    57,
                                                ),
                                                Regular(
                                                    60,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                10..16,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 23,
                                        },
                                        Eval {
                                            expr_idx: 42,
                                        },
                                        Break,
                                        Eval {
                                            expr_idx: 53,
                                        },
                                        Eval {
                                            expr_idx: 11,
                                        },
                                        While {
                                            condition: 20,
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 26,
                                        },
                                        While {
                                            condition: 39,
                                            stmts: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 45,
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        While {
                                            condition: 8,
                                            stmts: ArenaIdxRange(
                                                5..10,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 61,
                                        },
                                        Eval {
                                            expr_idx: 62,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 199,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 200,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 201,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            13,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                4,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                65,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                31,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd: 4,
                                        },
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                99,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                2,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                83,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                782,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        List {
                                            items: [
                                                1,
                                                2,
                                                3,
                                                5,
                                                6,
                                                7,
                                                8,
                                                9,
                                                10,
                                                11,
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 30,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 12,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 203,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            String(
                                                StringLiteral(
                                                    Id {
                                                        value: 18,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            String(
                                                StringLiteral(
                                                    Id {
                                                        value: 19,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            String(
                                                StringLiteral(
                                                    Id {
                                                        value: 20,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            String(
                                                StringLiteral(
                                                    Id {
                                                        value: 21,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            String(
                                                StringLiteral(
                                                    Id {
                                                        value: 22,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            String(
                                                StringLiteral(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        ),
                                        List {
                                            items: [
                                                1,
                                                2,
                                                3,
                                                4,
                                                5,
                                                6,
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 205,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]