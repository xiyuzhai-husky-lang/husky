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
                    body: Some(
                        Eager(
                            5,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 47,
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
                    body: Some(
                        Eager(
                            6,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 48,
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
                    body: Some(
                        Lazy(
                            108,
                        ),
                    ),
                    hir_expr_region: Lazy(
                        HirLazyExprRegion(
                            Id {
                                value: 2,
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
                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
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
                                                value: 386,
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
                                                value: 386,
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
                                                value: 386,
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
                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        34,
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
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
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
                                                value: 386,
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
                                            3.0,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 8,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 10,
                                },
                                MethodCall {
                                    self_argument: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 57,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
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
                                    lopd: 12,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 13,
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
                                    owner: 15,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 298,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 16,
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
                                            0.6,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 17,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 18,
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
                                    owner: 20,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 217,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 21,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 289,
                                            },
                                        ),
                                    ),
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
                                MethodCall {
                                    self_argument: 23,
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
                                MethodCall {
                                    self_argument: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 367,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            24,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 482,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 26,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
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
                                    lopd: 27,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 28,
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
                                MethodCall {
                                    self_argument: 30,
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
                                    owner: 31,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Minus,
                                    opd: 32,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..7,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Require {
                                    condition: 14,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 1,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 6,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Require {
                                    condition: 19,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 25,
                                },
                                Require {
                                    condition: 29,
                                },
                                Eval {
                                    expr_idx: 33,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 482,
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
]