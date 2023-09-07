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
                    body: Some(
                        Eager(
                            5,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 57,
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
                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        12,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 348,
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
                                    lopd: 1,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 2,
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
                                    opd: 4,
                                },
                                Binary {
                                    lopd: 3,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 5,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 348,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Minus,
                                    opd: 8,
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 9,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 10,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..2,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Require {
                                    condition: 6,
                                },
                                Eval {
                                    expr_idx: 11,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                    },
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
                    body: Some(
                        Lazy(
                            85,
                        ),
                    ),
                    hir_expr_region: Lazy(
                        HirLazyExprRegion(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]