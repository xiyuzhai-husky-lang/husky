[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                value: 23,
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
                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                            28,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 24,
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
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        21,
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
                                Field {
                                    owner: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 299,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
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
                                            0.5,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 2,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 3,
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
                                    owner: 5,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 141,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 7,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
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
                                    owner: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 12,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 141,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 13,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 14,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 15,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 10,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 16,
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
                                    owner: 18,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 299,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 19,
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
                                Block {
                                    stmts: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Require {
                                    condition: 17,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 4,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 20,
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
]