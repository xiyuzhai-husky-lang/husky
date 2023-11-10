[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValFugitiveHirDefn {
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
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                8,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 71,
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
                    path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `FunctionFn`),
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
                            9,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodCall {
                                            self_argument: 1,
                                            ident: `displacement`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 5,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 7,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Require {
                                            condition: HirEagerCondition(
                                                6,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 8,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
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
                                            HirEagerVariable {
                                                name: Ident(
                                                    Ident(
                                                        Coword(
                                                            Id {
                                                                value: 387,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                data: LetVariable,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `FunctionFn`),
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
                            9,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodCall {
                                            self_argument: 1,
                                            ident: `displacement`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 5,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 7,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Require {
                                            condition: HirEagerCondition(
                                                6,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 8,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
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
                                            HirEagerVariable {
                                                name: Ident(
                                                    Ident(
                                                        Coword(
                                                            Id {
                                                                value: 387,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                data: LetVariable,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `FunctionFn`),
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
                            9,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodCall {
                                            self_argument: 1,
                                            ident: `displacement`,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 5,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 7,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Require {
                                            condition: HirEagerCondition(
                                                6,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 8,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
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
                                            HirEagerVariable {
                                                name: Ident(
                                                    Ident(
                                                        Coword(
                                                            Id {
                                                                value: 387,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                data: LetVariable,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValFugitiveHirDefn {
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
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                120,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 73,
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