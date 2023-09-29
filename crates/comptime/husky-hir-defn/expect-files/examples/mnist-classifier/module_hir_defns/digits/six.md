[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                6,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                7,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Lazy(
                                109,
                            ),
                            Lazy(
                                HirLazyExprRegion(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
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
                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 303,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 3,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 5,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 7,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
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
                                        Require {
                                            condition: 6,
                                        },
                                        Eval {
                                            expr_idx: 8,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
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
                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            35,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 303,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 3,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    3.0,
                                                ),
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 5,
                                        },
                                        Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 6,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 8,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 9,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 11,
                                        },
                                        MethodCall {
                                            self_argument: 12,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    1.4,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 13,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 14,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 16,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 301,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 17,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.6,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 18,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 19,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 21,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 220,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 22,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 292,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 24,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 151,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        MethodCall {
                                            self_argument: 23,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 369,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    25,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 484,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 27,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.5,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 28,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 29,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 31,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 151,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Field {
                                            owner_hir_expr_idx: 32,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 33,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                2..8,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Require {
                                            condition: 15,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 7,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Require {
                                            condition: 20,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 26,
                                        },
                                        Require {
                                            condition: 30,
                                        },
                                        Eval {
                                            expr_idx: 34,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 484,
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