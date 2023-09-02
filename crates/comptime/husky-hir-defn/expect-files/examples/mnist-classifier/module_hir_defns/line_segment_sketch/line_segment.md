[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `start`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `end`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                ),
                            },
                        ],
                        hir_expr_region: HirEagerExprRegion {
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 58,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 58,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 47,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        5,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                SelfValue,
                                Field {
                                    owner: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 306,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            3,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 4,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `dist_to_point`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `dist_to_point`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 58,
                                },
                            ),
                        ),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 46,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        31,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                SelfValue,
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
                                SelfValue,
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 429,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 3,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 306,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            4,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 430,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 431,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 355,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            7,
                                        ),
                                    ],
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 431,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 352,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                SelfValue,
                                Field {
                                    owner: 13,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 429,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 14,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 306,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            15,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 430,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 432,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 17,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 355,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            18,
                                        ),
                                    ],
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            0.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 19,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 20,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 432,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 352,
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
                                                value: 430,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 431,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 24,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 356,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            25,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 26,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 430,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 28,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 352,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 27,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 29,
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        5..8,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 12,
                                },
                                Eval {
                                    expr_idx: 23,
                                },
                                Eval {
                                    expr_idx: 30,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 16,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 21,
                                        stmts: ArenaIdxRange(
                                            1..2,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 1,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 5,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 10,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                3..5,
                                            ),
                                        },
                                    ),
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 430,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 431,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 432,
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