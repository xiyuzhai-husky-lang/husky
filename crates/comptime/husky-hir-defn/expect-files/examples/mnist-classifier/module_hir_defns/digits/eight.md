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
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                6,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 23,
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
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                29,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 24,
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
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            22,
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
                                        Field {
                                            owner: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 301,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
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
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 4,
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
                                            owner: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 376,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 7,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 145,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [],
                                        },
                                        Suffix {
                                            opd: 8,
                                            opr: UnwrapOrComposeWithNot,
                                        },
                                        Field {
                                            owner: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 150,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
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
                                        Field {
                                            owner: 12,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 376,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 13,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 145,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [],
                                        },
                                        Suffix {
                                            opd: 14,
                                            opr: UnwrapOrComposeWithNot,
                                        },
                                        Field {
                                            owner: 15,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 151,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner: 16,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 11,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 17,
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
                                            owner: 19,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 301,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 20,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                2..4,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Require {
                                            condition: 18,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 5,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 21,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]