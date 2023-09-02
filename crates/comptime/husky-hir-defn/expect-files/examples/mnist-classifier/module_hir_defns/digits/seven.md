[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                value: 41,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        9,
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
                                Prefix {
                                    opr: Minus,
                                    opd: 7,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                            6,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 43,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        14,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 285,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 299,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 296,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
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
                                    lopd: 8,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 9,
                                },
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
                                    self_argument: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Field {
                                    owner: 12,
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
                                        0..4,
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
                                Require {
                                    condition: 10,
                                },
                                Eval {
                                    expr_idx: 13,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        19,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 285,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 299,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 295,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.3,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 8,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 9,
                                },
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
                                    self_argument: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 416,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                                MethodCall {
                                    self_argument: 12,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 357,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            13,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 511,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            30.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 15,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 16,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 511,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..6,
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
                                Require {
                                    condition: 10,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 14,
                                },
                                Require {
                                    condition: 17,
                                },
                                Eval {
                                    expr_idx: 18,
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
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 511,
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
                    path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                            60,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 46,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]