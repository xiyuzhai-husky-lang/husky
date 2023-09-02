[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                            7,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 54,
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
                    path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        8,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 285,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
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
                                    lopd: 3,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 4,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..3,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 1,
                                },
                                Require {
                                    condition: 5,
                                },
                                Eval {
                                    expr_idx: 7,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        8,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 285,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
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
                                    lopd: 3,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 4,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..3,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 1,
                                },
                                Require {
                                    condition: 5,
                                },
                                Eval {
                                    expr_idx: 7,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        8,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 285,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 301,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
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
                                    lopd: 3,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 4,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..3,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 1,
                                },
                                Require {
                                    condition: 5,
                                },
                                Eval {
                                    expr_idx: 7,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                            ],
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
                    path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                            119,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 56,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]