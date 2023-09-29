[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `row_start`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `row_end`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `upper_mass`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `lower_mass`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `matches`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 34,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 252,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 253,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 156,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    4.0,
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
                                                        value: 156,
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
                                            lopd: 6,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 7,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
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
                                                        value: 156,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `mask`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 35,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            43,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
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
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 3,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 4,
                                        },
                                        Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 5,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 7,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 8,
                                        },
                                        Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 9,
                                        },
                                        Binary {
                                            lopd: 1,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 10,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 14,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 15,
                                        },
                                        Binary {
                                            lopd: 13,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 16,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 18,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 19,
                                        },
                                        Binary {
                                            lopd: 17,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 20,
                                        },
                                        Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 21,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 278,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 23,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 24,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 278,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 26,
                                            opr: Assign,
                                            ropd: 27,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 278,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 32,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 33,
                                        },
                                        Binary {
                                            lopd: 31,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 34,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 36,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 37,
                                        },
                                        Binary {
                                            lopd: 35,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 38,
                                        },
                                        Binary {
                                            lopd: 30,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 39,
                                        },
                                        Binary {
                                            lopd: 29,
                                            opr: Assign,
                                            ropd: 40,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                3..7,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 28,
                                        },
                                        Eval {
                                            expr_idx: 41,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 22,
                                        },
                                        While {
                                            condition: 25,
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                        Return {
                                            result: 42,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 277,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 278,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            106,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        NewList {
                                            items: [],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 280,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 5,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Literal(
                                            I32(
                                                30,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 281,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 273,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 5,
                                            items: [
                                                6,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 281,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 273,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 8,
                                            items: [
                                                9,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 11,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 129,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        AssociatedFn,
                                        FnCall {
                                            function_hir_expr_idx: 13,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 273,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 15,
                                            items: [
                                                16,
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 282,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 20,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 21,
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 18,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    19,
                                                ),
                                                Regular(
                                                    22,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 17,
                                            opr: Assign,
                                            ropd: 23,
                                        },
                                        Literal(
                                            Bool(
                                                false,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 26,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            Bool(
                                                true,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 28,
                                            opr: Assign,
                                            ropd: 29,
                                        },
                                        CurrentSymbol {
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
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 33,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 34,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 32,
                                            items: [
                                                35,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 280,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 40,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 41,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 39,
                                            items: [
                                                42,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 44,
                                            items: [
                                                45,
                                            ],
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 38,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    43,
                                                ),
                                                Regular(
                                                    46,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 37,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 47,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 49,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 51,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 52,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            Bool(
                                                false,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 54,
                                            opr: Assign,
                                            ropd: 55,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 58,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 59,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 57,
                                            items: [
                                                60,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 61,
                                            opr: Assign,
                                            ropd: 62,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 64,
                                            items: [
                                                65,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 8,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 280,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 69,
                                            items: [
                                                70,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 73,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 74,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 72,
                                            items: [
                                                75,
                                            ],
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 68,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    71,
                                                ),
                                                Regular(
                                                    76,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 67,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 77,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 79,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 80,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            Bool(
                                                false,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 82,
                                            opr: Assign,
                                            ropd: 83,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 85,
                                            items: [
                                                86,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 87,
                                            opr: Assign,
                                            ropd: 88,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 273,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                30,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 281,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 128,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 92,
                                            items: [
                                                93,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 128,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 95,
                                            items: [
                                                96,
                                            ],
                                        },
                                        Prefix {
                                            opr: BitNot,
                                            opd_hir_expr_idx: 97,
                                        },
                                        Binary {
                                            lopd: 94,
                                            opr: AssignClosed(
                                                BitAnd,
                                            ),
                                            ropd: 98,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 47,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 101,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    102,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 100,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 143,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    103,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                27..31,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Break,
                                        Eval {
                                            expr_idx: 56,
                                        },
                                        Eval {
                                            expr_idx: 63,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            initial_value: 36,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            initial_value: 48,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 50,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 53,
                                                stmts: ArenaIdxRange(
                                                    2..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 84,
                                        },
                                        Eval {
                                            expr_idx: 89,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            initial_value: 66,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 11,
                                                ty: None,
                                            },
                                            initial_value: 78,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 81,
                                                stmts: ArenaIdxRange(
                                                    8..10,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 30,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: None,
                                            },
                                            initial_value: 31,
                                        },
                                        ForExt {
                                            particulars: HirEagerForExtParticulars,
                                            block: ArenaIdxRange(
                                                4..8,
                                            ),
                                        },
                                        ForExt {
                                            particulars: HirEagerForExtParticulars,
                                            block: ArenaIdxRange(
                                                10..13,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 99,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 10,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 14,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 25,
                                        },
                                        While {
                                            condition: 27,
                                            stmts: ArenaIdxRange(
                                                13..17,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 128,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            90,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            91,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                17..18,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 104,
                                        },
                                        While {
                                            condition: 7,
                                            stmts: ArenaIdxRange(
                                                18..26,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 37,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 1,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 273,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                26..27,
                                            ),
                                        },
                                        Return {
                                            result: 105,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 17,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 281,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 282,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
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
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::connected_component`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::visual::Visualize`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 36,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        self_value_parameter: HirRitchieParameter {
                            contract: None,
                            ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 36,
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
                                    value: 29,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 160,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
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
                                            expr_idx: 3,
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
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::connected_component`,
                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 36,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `raw_contours`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `raw_contours`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 38,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 17,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        SelfType,
                                        FnCall {
                                            function_hir_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    2,
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
                                            expr_idx: 3,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `eff_holes`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `eff_holes`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            21,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 148,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        NewList {
                                            items: [],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        MethodCall {
                                            self_argument: 5,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 153,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    6,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 249,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        MethodCall {
                                            self_argument: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 153,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    10,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 8,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 143,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    11,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 249,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 7,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        MethodCall {
                                            self_argument: 14,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 153,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    15,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 13,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 143,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    16,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 249,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 18,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    19,
                                                ),
                                            ],
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
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 34,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 4,
                                        },
                                        Eval {
                                            expr_idx: 7,
                                        },
                                        Eval {
                                            expr_idx: 12,
                                        },
                                        Eval {
                                            expr_idx: 17,
                                        },
                                        Return {
                                            result: 20,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 249,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `max_hole_ilen`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `max_hole_ilen`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            21,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 5,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 142,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                8,
                                            ],
                                        },
                                        Field {
                                            owner_hir_expr_idx: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 263,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 142,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 12,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 13,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 15,
                                            opr: Assign,
                                            ropd: 16,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 18,
                                            opr: As,
                                            ropd: 19,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                4..8,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 17,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 14,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 261,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            6,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                2..4,
                                            ),
                                        },
                                        Return {
                                            result: 20,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 257,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `max_row_span`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `max_row_span`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            16,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                29,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 265,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 265,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                8,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 131,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        MethodCall {
                                            self_argument: 5,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 62,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    10,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 4,
                                            opr: Assign,
                                            ropd: 11,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 265,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 13,
                                            opr: As,
                                            ropd: 14,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                2..5,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 12,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 5,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 1,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 261,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            2,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            3,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Return {
                                            result: 15,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 265,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `row_span_sum`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `row_span_sum`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            14,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                29,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 266,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 5,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 6,
                                            items: [
                                                7,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 8,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 131,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 4,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 9,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 266,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 11,
                                            opr: As,
                                            ropd: 12,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                2..5,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 10,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 261,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            2,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            3,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Return {
                                            result: 13,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 266,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `distribution`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `distribution`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 40,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            50,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 3,
                                            items: [
                                                4,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 7,
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 245,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 10,
                                            items: [
                                                11,
                                            ],
                                        },
                                        Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 12,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 245,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 14,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 15,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 268,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                2,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 17,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 18,
                                        },
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 269,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 23,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 246,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 26,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 270,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 27,
                                            items: [
                                                28,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 29,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 130,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 25,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 30,
                                        },
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 245,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 245,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 269,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 34,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 35,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 247,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 38,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 271,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 39,
                                            items: [
                                                40,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 41,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 130,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 37,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 42,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 45,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 245,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 246,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 247,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 44,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    45,
                                                ),
                                                Regular(
                                                    46,
                                                ),
                                                Regular(
                                                    47,
                                                ),
                                                Regular(
                                                    48,
                                                ),
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                7..18,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Break,
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
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 13,
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                        },
                                        Eval {
                                            expr_idx: 43,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        ForExt {
                                            particulars: HirEagerForExtParticulars,
                                            block: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        ForExt {
                                            particulars: HirEagerForExtParticulars,
                                            block: ArenaIdxRange(
                                                4..5,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 16,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 19,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 20,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 270,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            21,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            24,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                5..6,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 32,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 271,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            33,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            36,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    step: Constant(
                                                        -1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                6..7,
                                            ),
                                        },
                                        Return {
                                            result: 49,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 244,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 245,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 268,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 269,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 246,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 247,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `upper_mass`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `upper_mass`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 267,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 246,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 3,
                                            opr: As,
                                            ropd: 4,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `lower_mass`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `lower_mass`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 267,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 247,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 3,
                                            opr: As,
                                            ropd: 4,
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
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `top_k_row_span_sum`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `top_k_row_span_sum`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 36,
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
                                        value: 36,
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
                                                value: 5,
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
                            24,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 128,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 2,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 3,
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                8,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 128,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 11,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 12,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 15,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 273,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 16,
                                            items: [
                                                17,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 18,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 131,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 14,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 19,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 21,
                                            opr: As,
                                            ropd: 22,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                4..10,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 9,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 20,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        Assert {
                                            condition: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 5,
                                        },
                                        ForExt {
                                            particulars: HirEagerForExtParticulars,
                                            block: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 273,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            10,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            13,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                        Return {
                                            result: 23,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `top_k_row_right_mass_sum`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `top_k_row_right_mass_sum`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 36,
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
                                        value: 36,
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
                                                value: 5,
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
                            24,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 128,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 2,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 3,
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                8,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 128,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 11,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 12,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 15,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 255,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 273,
                                                    },
                                                ),
                                            ),
                                        },
                                        Index {
                                            owner_hir_expr_idx: 16,
                                            items: [
                                                17,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 18,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 132,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 14,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 19,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 21,
                                            opr: As,
                                            ropd: 22,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                4..10,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 9,
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 20,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        Assert {
                                            condition: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 5,
                                        },
                                        ForExt {
                                            particulars: HirEagerForExtParticulars,
                                            block: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 273,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            10,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            13,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                        Return {
                                            result: 23,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 261,
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