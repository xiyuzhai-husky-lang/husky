[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                        value: 57,
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            13,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 349,
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
                                            lopd: 2,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 3,
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    140.0,
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
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 8,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 349,
                                                    },
                                                ),
                                            ),
                                        },
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 9,
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 10,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 11,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Require {
                                            condition: 7,
                                        },
                                        Eval {
                                            expr_idx: 12,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
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
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                86,
                            ),
                            Lazy(
                                HirLazyExprRegion(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
]