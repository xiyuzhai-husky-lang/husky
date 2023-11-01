[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `cc`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 65,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `points`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 66,
                                        },
                                    ),
                                ),
                            },
                        ],
                        hir_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        hir_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 312,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
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
                                            lopd: 2,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 3,
                                        },
                                        Binary {
                                            lopd: 1,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 4,
                                        },
                                        Literal(
                                            R32(
                                                3,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 6,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 7,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
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
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 312,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 1,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 2,
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 4,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
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
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 312,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
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
                                            lopd: 2,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 3,
                                        },
                                        Binary {
                                            lopd: 1,
                                            opr: Shift(
                                                Shr,
                                            ),
                                            ropd: 4,
                                        },
                                        Literal(
                                            R32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 6,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 7,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
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
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 316,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    2,
                                                ),
                                                Regular(
                                                    3,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 317,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 5,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    6,
                                                ),
                                                Regular(
                                                    7,
                                                ),
                                            ],
                                        },
                                        Block {
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
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Match,
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 319,
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 322,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 18,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 1,
                                            opr: As,
                                            ropd: 2,
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 321,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 18,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 4,
                                            opr: As,
                                            ropd: 5,
                                        },
                                        Binary {
                                            lopd: 3,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 6,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 32,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 7,
                                            opr: As,
                                            ropd: 8,
                                        },
                                        Literal(
                                            I32(
                                                2,
                                            ),
                                        ),
                                        MethodCall {
                                            self_argument: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 127,
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
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..3,
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
                                            initial_value: 11,
                                        },
                                        Match,
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 323,
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 316,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    2,
                                                ),
                                                Regular(
                                                    3,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 10,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 317,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 5,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    6,
                                                ),
                                                Regular(
                                                    7,
                                                ),
                                            ],
                                        },
                                        Block {
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
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Match,
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 318,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 319,
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
                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `prev1`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `prev2`,
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
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            29,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 1,
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
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
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
                                            lopd: 4,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 5,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 3,
                                            items: [
                                                6,
                                            ],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
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
                                            lopd: 9,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 10,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 8,
                                            items: [
                                                11,
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 51,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 330,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 14,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 331,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 16,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 15,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 17,
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    2.0,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 19,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 330,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 21,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 331,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 23,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 24,
                                        },
                                        Literal(
                                            F32(
                                                NotNan(
                                                    2.0,
                                                ),
                                            ),
                                        ),
                                        Binary {
                                            lopd: 25,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 26,
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 13,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    20,
                                                ),
                                                Regular(
                                                    27,
                                                ),
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..5,
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
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        Eval {
                                            expr_idx: 28,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 330,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 331,
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            267,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        NewList {
                                            items: [],
                                        },
                                        AssociatedFn,
                                        FnCall {
                                            function_hir_expr_idx: 2,
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                29,
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 254,
                                                    },
                                                ),
                                            ),
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
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 9,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                10,
                                            ],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 12,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 254,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 13,
                                            items: [
                                                14,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 334,
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
                                            lopd: 16,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 17,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 335,
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
                                            lopd: 19,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 20,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 332,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 22,
                                            items: [
                                                23,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 334,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 335,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 25,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 26,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 336,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 27,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 28,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 337,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 29,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 30,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 334,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 335,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 32,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 33,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 336,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 34,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 35,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 337,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 36,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 37,
                                        },
                                        Prefix {
                                            opr: BitNot,
                                            opd_hir_expr_idx: 38,
                                        },
                                        Binary {
                                            lopd: 31,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 39,
                                        },
                                        Binary {
                                            lopd: 24,
                                            opr: Assign,
                                            ropd: 40,
                                        },
                                        Literal(
                                            I32(
                                                1,
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
                                                        value: 332,
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
                                            owner_hir_expr_idx: 44,
                                            items: [
                                                45,
                                            ],
                                        },
                                        NewList {
                                            items: [],
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
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 332,
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
                                            owner_hir_expr_idx: 49,
                                            items: [
                                                50,
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 51,
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
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 53,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 254,
                                                    },
                                                ),
                                            ),
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
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 55,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 56,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 54,
                                            items: [
                                                57,
                                            ],
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 59,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 254,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 60,
                                            items: [
                                                61,
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 13,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 316,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 317,
                                                    },
                                                ),
                                            ),
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
                                        FnCall {
                                            function_hir_expr_idx: 63,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    64,
                                                ),
                                                Regular(
                                                    65,
                                                ),
                                                Regular(
                                                    66,
                                                ),
                                            ],
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
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
                                                0,
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 74,
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 76,
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 78,
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
                                                        value: 339,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 80,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 81,
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
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 340,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 83,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 84,
                                        },
                                        Binary {
                                            lopd: 82,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 85,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 341,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 87,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 88,
                                        },
                                        Binary {
                                            lopd: 86,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 89,
                                        },
                                        Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 90,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 15,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 316,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 317,
                                                    },
                                                ),
                                            ),
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
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 92,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    93,
                                                ),
                                                Regular(
                                                    94,
                                                ),
                                                Regular(
                                                    95,
                                                ),
                                                Regular(
                                                    96,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 14,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 348,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 98,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    99,
                                                ),
                                                Regular(
                                                    100,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 332,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 102,
                                            items: [
                                                103,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 332,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 105,
                                            items: [
                                                106,
                                            ],
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 108,
                                            opr: Shift(
                                                Shl,
                                            ),
                                            ropd: 109,
                                        },
                                        Prefix {
                                            opr: BitNot,
                                            opd_hir_expr_idx: 110,
                                        },
                                        Binary {
                                            lopd: 107,
                                            opr: Closed(
                                                BitOr,
                                            ),
                                            ropd: 111,
                                        },
                                        Binary {
                                            lopd: 104,
                                            opr: Assign,
                                            ropd: 112,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 349,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 116,
                                        },
                                        Binary {
                                            lopd: 115,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 117,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 343,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 120,
                                        },
                                        Binary {
                                            lopd: 119,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 121,
                                        },
                                        Binary {
                                            lopd: 118,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 122,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
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
                                            lopd: 124,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 125,
                                        },
                                        Binary {
                                            lopd: 123,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 126,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 129,
                                        },
                                        Binary {
                                            lopd: 128,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 130,
                                        },
                                        Binary {
                                            lopd: 127,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 131,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 346,
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
                                            lopd: 133,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 134,
                                        },
                                        Binary {
                                            lopd: 132,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 135,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 137,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 146,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Suffix {
                                            opd_hir_expr_idx: 138,
                                            opr: Unwrap,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Fugitive(
                                                    FugitivePath(
                                                        Id {
                                                            value: 16,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 140,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    141,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 139,
                                            opr: Assign,
                                            ropd: 142,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        AssociatedFn,
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 145,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    146,
                                                ),
                                                Regular(
                                                    147,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 144,
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
                                                    148,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 346,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 151,
                                        },
                                        Binary {
                                            lopd: 150,
                                            opr: Assign,
                                            ropd: 152,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 155,
                                        },
                                        Binary {
                                            lopd: 154,
                                            opr: Assign,
                                            ropd: 156,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 159,
                                        },
                                        Binary {
                                            lopd: 158,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 160,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
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
                                            lopd: 162,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 163,
                                        },
                                        Binary {
                                            lopd: 161,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 164,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
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
                                            lopd: 166,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 167,
                                        },
                                        Binary {
                                            lopd: 165,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 168,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 170,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 146,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Suffix {
                                            opd_hir_expr_idx: 171,
                                            opr: Unwrap,
                                        },
                                        AssociatedFn,
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 173,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    174,
                                                ),
                                                Regular(
                                                    175,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 172,
                                            opr: Assign,
                                            ropd: 176,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 346,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 178,
                                            opr: Assign,
                                            ropd: 179,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 181,
                                            opr: Assign,
                                            ropd: 182,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 185,
                                        },
                                        Binary {
                                            lopd: 184,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 186,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
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
                                            lopd: 188,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 189,
                                        },
                                        Binary {
                                            lopd: 187,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 190,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
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
                                            lopd: 192,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 193,
                                        },
                                        Binary {
                                            lopd: 191,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 194,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
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
                                            lopd: 196,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 197,
                                        },
                                        Binary {
                                            lopd: 195,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 198,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 200,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 146,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Suffix {
                                            opd_hir_expr_idx: 201,
                                            opr: Unwrap,
                                        },
                                        AssociatedFn,
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 203,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    204,
                                                ),
                                                Regular(
                                                    205,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 202,
                                            opr: Assign,
                                            ropd: 206,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 346,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 209,
                                        },
                                        Binary {
                                            lopd: 208,
                                            opr: Assign,
                                            ropd: 210,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 213,
                                        },
                                        Binary {
                                            lopd: 212,
                                            opr: Assign,
                                            ropd: 214,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        AssociatedFn,
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 217,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    218,
                                                ),
                                                Regular(
                                                    219,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 216,
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
                                                    220,
                                                ),
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 346,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 222,
                                            opr: Assign,
                                            ropd: 223,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 225,
                                            opr: Assign,
                                            ropd: 226,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
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
                                            lopd: 228,
                                            opr: Assign,
                                            ropd: 229,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 343,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 231,
                                            opr: Assign,
                                            ropd: 232,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 349,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 234,
                                            opr: Assign,
                                            ropd: 235,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 348,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 237,
                                            opr: Assign,
                                            ropd: 238,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 241,
                                        },
                                        Binary {
                                            lopd: 240,
                                            opr: Comparison(
                                                Neq,
                                            ),
                                            ropd: 242,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
                                                    },
                                                ),
                                            ),
                                        },
                                        Suffix {
                                            opd_hir_expr_idx: 244,
                                            opr: Incr,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 247,
                                        },
                                        Binary {
                                            lopd: 246,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 248,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
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
                                            lopd: 250,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            ropd: 251,
                                        },
                                        Binary {
                                            lopd: 249,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 252,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
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
                                            lopd: 254,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 255,
                                        },
                                        Binary {
                                            lopd: 253,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 256,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 258,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 147,
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
                                                            value: 48,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 261,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    262,
                                                ),
                                                Regular(
                                                    263,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 260,
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
                                                    264,
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
                                                51..56,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 15,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 18,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 21,
                                        },
                                        Eval {
                                            expr_idx: 41,
                                        },
                                        Eval {
                                            expr_idx: 143,
                                        },
                                        Eval {
                                            expr_idx: 149,
                                        },
                                        Eval {
                                            expr_idx: 153,
                                        },
                                        Eval {
                                            expr_idx: 157,
                                        },
                                        Eval {
                                            expr_idx: 177,
                                        },
                                        Eval {
                                            expr_idx: 180,
                                        },
                                        Eval {
                                            expr_idx: 183,
                                        },
                                        Eval {
                                            expr_idx: 207,
                                        },
                                        Eval {
                                            expr_idx: 211,
                                        },
                                        Eval {
                                            expr_idx: 215,
                                        },
                                        Eval {
                                            expr_idx: 221,
                                        },
                                        Eval {
                                            expr_idx: 224,
                                        },
                                        Eval {
                                            expr_idx: 227,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 136,
                                                stmts: ArenaIdxRange(
                                                    6..10,
                                                ),
                                            },
                                            elif_branches: [
                                                HirEagerElifBranch {
                                                    condition: 169,
                                                    stmts: ArenaIdxRange(
                                                        10..13,
                                                    ),
                                                },
                                                HirEagerElifBranch {
                                                    condition: 199,
                                                    stmts: ArenaIdxRange(
                                                        13..16,
                                                    ),
                                                },
                                            ],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        16..19,
                                                    ),
                                                },
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 230,
                                        },
                                        Eval {
                                            expr_idx: 233,
                                        },
                                        Eval {
                                            expr_idx: 236,
                                        },
                                        Eval {
                                            expr_idx: 245,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 22,
                                                ty: None,
                                            },
                                            initial_value: 97,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 23,
                                                ty: None,
                                            },
                                            initial_value: 101,
                                        },
                                        Eval {
                                            expr_idx: 113,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 114,
                                                stmts: ArenaIdxRange(
                                                    19..23,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Match,
                                        Eval {
                                            expr_idx: 239,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 243,
                                                stmts: ArenaIdxRange(
                                                    23..24,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 259,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 66,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 47,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            initial_value: 48,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            initial_value: 52,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            initial_value: 58,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 11,
                                                ty: None,
                                            },
                                            initial_value: 62,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 12,
                                                ty: None,
                                            },
                                            initial_value: 67,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 13,
                                                ty: None,
                                            },
                                            initial_value: 68,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 14,
                                                ty: None,
                                            },
                                            initial_value: 69,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 15,
                                                ty: None,
                                            },
                                            initial_value: 70,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 16,
                                                ty: None,
                                            },
                                            initial_value: 71,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 17,
                                                ty: None,
                                            },
                                            initial_value: 72,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 18,
                                                ty: None,
                                            },
                                            initial_value: 73,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 19,
                                                ty: None,
                                            },
                                            initial_value: 75,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 20,
                                                ty: None,
                                            },
                                            initial_value: 77,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 21,
                                                ty: None,
                                            },
                                            initial_value: 79,
                                        },
                                        DoWhile {
                                            condition: 91,
                                            block: ArenaIdxRange(
                                                24..31,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: 257,
                                                stmts: ArenaIdxRange(
                                                    31..32,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 265,
                                        },
                                        While {
                                            condition: 46,
                                            stmts: ArenaIdxRange(
                                                32..50,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 38,
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
                                                            value: 260,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            5,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..6,
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
                                                            42,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            43,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                50..51,
                                            ),
                                        },
                                        Return {
                                            result: 266,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
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
                                                        value: 332,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 334,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 335,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 336,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 337,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 338,
                                                    },
                                                ),
                                            ),
                                        },
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
                                                        value: 272,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 316,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 317,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 325,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 339,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 340,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 341,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 343,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 344,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 345,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 346,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 347,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 348,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 349,
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
        ImplBlockHirDefn {
            hir_decl: TraitForType(
                TraitForTypeImplBlockHirDecl(
                    Id {
                        value: 23,
                    },
                ),
            ),
        },
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                        value: 31,
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
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        EmptyHtmlTag {
                                            function_ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 288,
                                                    },
                                                ),
                                            ),
                                            arguments: [
                                                HirEagerHtmlArgumentExpr {
                                                    property_ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 262,
                                                            },
                                                        ),
                                                    ),
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
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
        ImplBlockHirDefn {
            hir_decl: Type(
                TypeImplBlockHirDecl(
                    Id {
                        value: 30,
                    },
                ),
            ),
        },
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `line_segment_sketch`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `line_segment_sketch`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 54,
                                },
                            ),
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            5,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        AssociatedFn,
                                        SelfType,
                                        Literal(
                                            F32(
                                                NotNan(
                                                    1.4,
                                                ),
                                            ),
                                        ),
                                        FnCall {
                                            function_hir_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    2,
                                                ),
                                                Regular(
                                                    3,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
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
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `bounding_box`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 51,
                                },
                            ),
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            54,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            USize(
                                                TermUSizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Index {
                                            owner_hir_expr_idx: 2,
                                            items: [
                                                3,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 293,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 5,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 293,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 7,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 293,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 9,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 293,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 11,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 13,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 14,
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
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 16,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 17,
                                            items: [
                                                18,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 294,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 294,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 22,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 21,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 67,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    23,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 20,
                                            opr: Assign,
                                            ropd: 24,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 295,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 295,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 28,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 27,
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
                                                    29,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 26,
                                            opr: Assign,
                                            ropd: 30,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 296,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 296,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 34,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 33,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 67,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    35,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 32,
                                            opr: Assign,
                                            ropd: 36,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 297,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 297,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 40,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 39,
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
                                                    41,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 38,
                                            opr: Assign,
                                            ropd: 42,
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 55,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 294,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 295,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 45,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    46,
                                                ),
                                                Regular(
                                                    47,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 296,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 297,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 49,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    50,
                                                ),
                                                Regular(
                                                    51,
                                                ),
                                            ],
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 44,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    48,
                                                ),
                                                Regular(
                                                    52,
                                                ),
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                6..13,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 19,
                                        },
                                        Eval {
                                            expr_idx: 25,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                        },
                                        Eval {
                                            expr_idx: 37,
                                        },
                                        Eval {
                                            expr_idx: 43,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 10,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 260,
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
                                                            15,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                        Return {
                                            result: 53,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 293,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 294,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 295,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 296,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 297,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
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
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `relative_bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `relative_bounding_box`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 52,
                                },
                            ),
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            10,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 256,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            USize(
                                                TermUSizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Index {
                                            owner_hir_expr_idx: 3,
                                            items: [
                                                4,
                                            ],
                                        },
                                        Field {
                                            owner_hir_expr_idx: 5,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 291,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 7,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 291,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 300,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    8,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 9,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
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
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `contour_len`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `contour_len`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            59,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        Literal(
                                            I32(
                                                0,
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 3,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 4,
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
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 6,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
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
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 9,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 7,
                                            items: [
                                                10,
                                            ],
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 12,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
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
                                        Index {
                                            owner_hir_expr_idx: 13,
                                            items: [
                                                14,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 252,
                                                    },
                                                ),
                                            ),
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
                                        Field {
                                            owner_hir_expr_idx: 17,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 158,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 19,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 20,
                                        },
                                        MethodCall {
                                            self_argument: 21,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 61,
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
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 23,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 158,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 25,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 24,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 26,
                                        },
                                        MethodCall {
                                            self_argument: 27,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 28,
                                        },
                                        Binary {
                                            lopd: 16,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 29,
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 31,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 33,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 34,
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
                                        Literal(
                                            I32(
                                                1,
                                            ),
                                        ),
                                        Binary {
                                            lopd: 35,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 36,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 32,
                                            items: [
                                                37,
                                            ],
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 39,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        Literal(
                                            USize(
                                                TermUSizeLiteral(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        ),
                                        Index {
                                            owner_hir_expr_idx: 40,
                                            items: [
                                                41,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 252,
                                                    },
                                                ),
                                            ),
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
                                        Field {
                                            owner_hir_expr_idx: 44,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 158,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 46,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 275,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 45,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 47,
                                        },
                                        MethodCall {
                                            self_argument: 48,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 61,
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
                                                        value: 53,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 50,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 158,
                                                    },
                                                ),
                                            ),
                                        },
                                        Field {
                                            owner_hir_expr_idx: 52,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 276,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 51,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 53,
                                        },
                                        MethodCall {
                                            self_argument: 54,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        Binary {
                                            lopd: 49,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 55,
                                        },
                                        Binary {
                                            lopd: 43,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 56,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 252,
                                                    },
                                                ),
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                4..10,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 15,
                                        },
                                        Eval {
                                            expr_idx: 30,
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
                                                            value: 260,
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
                                                            5,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 38,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 42,
                                        },
                                        Eval {
                                            expr_idx: 57,
                                        },
                                        Return {
                                            result: 58,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 252,
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
                                                        value: 158,
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
                                                        value: 158,
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
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 31,
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
                                        value: 31,
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
                                    value: 49,
                                },
                            ),
                        ),
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            19,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 1,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 2,
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
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 4,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 150,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            ropd: 7,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 5,
                                            items: [
                                                8,
                                            ],
                                        },
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 262,
                                                    },
                                                ),
                                            ),
                                        },
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 151,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
                                                    },
                                                ),
                                            ),
                                        },
                                        Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                RemEuclid,
                                            ),
                                            ropd: 13,
                                        },
                                        Index {
                                            owner_hir_expr_idx: 11,
                                            items: [
                                                14,
                                            ],
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 305,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 306,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 16,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 307,
                                                    },
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    17,
                                                ),
                                            ],
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                1..5,
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
                                            initial_value: 3,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 9,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 15,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 305,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 306,
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