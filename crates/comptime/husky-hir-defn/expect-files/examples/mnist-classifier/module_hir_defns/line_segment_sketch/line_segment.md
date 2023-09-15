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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        SelfValue,
                                        Field {
                                            owner: 1,
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
                                            owner: 3,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 147,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 305,
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
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            32,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        SelfValue,
                                        MethodCall {
                                            self_argument: 1,
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
                                        SelfValue,
                                        Field {
                                            owner: 3,
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
                                                        value: 428,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 4,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 305,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    5,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 429,
                                                    },
                                                ),
                                            ),
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
                                            self_argument: 7,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 354,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    8,
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
                                            lopd: 9,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 10,
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
                                            self_argument: 12,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 351,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [],
                                        },
                                        SelfValue,
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
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 428,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 15,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 305,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    16,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 429,
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
                                            self_argument: 18,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 354,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    19,
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
                                            lopd: 20,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 21,
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
                                            self_argument: 23,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 351,
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
                                                        value: 429,
                                                    },
                                                ),
                                            ),
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
                                            self_argument: 25,
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
                                                    26,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 27,
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
                                                        value: 429,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 29,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 351,
                                                    },
                                                ),
                                            ),
                                            generic_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 28,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 30,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                6..9,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 13,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 17,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 22,
                                                stmts: ArenaIdxRange(
                                                    2..3,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 11,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        4..6,
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
                                                        value: 429,
                                                    },
                                                ),
                                            ),
                                        },
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
                                    ],
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]