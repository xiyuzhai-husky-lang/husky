[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValFugitiveHirDefn {
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
                                        value: 39,
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
                ValFugitiveHirDefn {
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
                                        value: 40,
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
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 43,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    Type(
                                        PathLeading(
                                            HirTypePathLeading(
                                                Id {
                                                    value: 14,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 286,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_variable_region: HirEagerVariableRegion {
                                arena: Arena {
                                    data: [
                                        HirEagerVariable {
                                            name: Ident(
                                                Ident(
                                                    Coword(
                                                        Id {
                                                            value: 286,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            data: ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            22,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `relative_bounding_box`,
                                        },
                                        HirEagerExprData::MethodCall {
                                            self_argument: 2,
                                            ident: `ymax`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.5,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 6,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodCall {
                                            self_argument: 7,
                                            ident: `first`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 8,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 9,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 10,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 12,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodCall {
                                            self_argument: 13,
                                            ident: `first`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 14,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 15,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 16,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 11,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 17,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 19,
                                            ident: `relative_bounding_box`,
                                        },
                                        HirEagerExprData::MethodCall {
                                            self_argument: 20,
                                            ident: `ymax`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                2..4,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Require {
                                            condition: HirEagerCondition(
                                                18,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    5,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 21,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_variable_region: HirEagerVariableRegion {
                                    arena: Arena {
                                        data: [
                                            HirEagerVariable {
                                                name: Ident(
                                                    Ident(
                                                        Coword(
                                                            Id {
                                                                value: 286,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                data: ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]