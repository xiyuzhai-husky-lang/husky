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
                                            value: 63,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `points`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 64,
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
            TypeHirDefn::Enum(
                EnumTypeHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        7,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 310,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
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
                                    lopd: 1,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 2,
                                },
                                Binary {
                                    lopd: 0,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 3,
                                },
                                Literal(
                                    R32(
                                        3,
                                    ),
                                ),
                                Binary {
                                    lopd: 4,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 5,
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
                                    expr_idx: 6,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        5,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 310,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 0,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 1,
                                },
                                Literal(
                                    R32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 2,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 3,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        7,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 310,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
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
                                    lopd: 1,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 2,
                                },
                                Binary {
                                    lopd: 0,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 3,
                                },
                                Literal(
                                    R32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 4,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 5,
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
                                    expr_idx: 6,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
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
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 0,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            1,
                                        ),
                                        Regular(
                                            2,
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
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 4,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            5,
                                        ),
                                        Regular(
                                            6,
                                        ),
                                    ],
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
                                    initial_value: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 7,
                                },
                                Match,
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        11,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
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
                                                    value: 15,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 0,
                                    opr: As,
                                    ropd: 1,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 320,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 15,
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
                                Binary {
                                    lopd: 2,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 5,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 29,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 6,
                                    opr: As,
                                    ropd: 7,
                                },
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                                MethodCall {
                                    self_argument: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 123,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            9,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..2,
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
                                    initial_value: 10,
                                },
                                Match,
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 322,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
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
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 0,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            1,
                                        ),
                                        Regular(
                                            2,
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
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 4,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            5,
                                        ),
                                        Regular(
                                            6,
                                        ),
                                    ],
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
                                    initial_value: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 7,
                                },
                                Match,
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
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
                            ],
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        28,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                InheritedSymbol {
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
                                                value: 302,
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
                                    lopd: 3,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 4,
                                },
                                Index {
                                    owner: 2,
                                    items: [
                                        5,
                                    ],
                                },
                                InheritedSymbol {
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
                                                value: 302,
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
                                    owner: 7,
                                    items: [
                                        10,
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 49,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 329,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 13,
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
                                                value: 330,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 15,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 14,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 16,
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            2.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 17,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 18,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 329,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 20,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
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
                                    owner: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 21,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 23,
                                },
                                Literal(
                                    F32(
                                        NotNan(
                                            2.0,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 24,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 25,
                                },
                                FnCall {
                                    function: 12,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            19,
                                        ),
                                        Regular(
                                            26,
                                        ),
                                    ],
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
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 6,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 11,
                                },
                                Eval {
                                    expr_idx: 27,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 329,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        266,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                List {
                                    items: [],
                                },
                                AssociatedFn,
                                FnCall {
                                    function: 1,
                                    generic_arguments: None,
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
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 5,
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
                                                value: 258,
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
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 8,
                                },
                                Index {
                                    owner: 6,
                                    items: [
                                        9,
                                    ],
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
                                    owner: 11,
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
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 12,
                                    items: [
                                        13,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
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
                                    lopd: 15,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 16,
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
                                    lopd: 18,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 19,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 21,
                                    items: [
                                        22,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
                                            },
                                        ),
                                    ),
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
                                Binary {
                                    lopd: 24,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 25,
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
                                    lopd: 26,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 27,
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
                                    lopd: 28,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 29,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
                                            },
                                        ),
                                    ),
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
                                Binary {
                                    lopd: 31,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 32,
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
                                    lopd: 33,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 34,
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
                                    lopd: 35,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 36,
                                },
                                Prefix {
                                    opr: Tilde,
                                    opd: 37,
                                },
                                Binary {
                                    lopd: 30,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 38,
                                },
                                Binary {
                                    lopd: 23,
                                    opr: Assign,
                                    ropd: 39,
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
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 43,
                                    items: [
                                        44,
                                    ],
                                },
                                List {
                                    items: [],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 48,
                                    items: [
                                        49,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 50,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 125,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
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
                                    owner: 52,
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
                                                value: 258,
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
                                    lopd: 54,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 55,
                                },
                                Index {
                                    owner: 53,
                                    items: [
                                        56,
                                    ],
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
                                    owner: 58,
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
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 59,
                                    items: [
                                        60,
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
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
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
                                FnCall {
                                    function: 62,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            63,
                                        ),
                                        Regular(
                                            64,
                                        ),
                                        Regular(
                                            65,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
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
                                    opd: 73,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 75,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 77,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
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
                                Binary {
                                    lopd: 79,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 80,
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
                                    lopd: 82,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 83,
                                },
                                Binary {
                                    lopd: 81,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 84,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
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
                                    lopd: 86,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 87,
                                },
                                Binary {
                                    lopd: 85,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 88,
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 89,
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
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 91,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            92,
                                        ),
                                        Regular(
                                            93,
                                        ),
                                        Regular(
                                            94,
                                        ),
                                        Regular(
                                            95,
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
                                                value: 324,
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
                                FnCall {
                                    function: 97,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            98,
                                        ),
                                        Regular(
                                            99,
                                        ),
                                    ],
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 101,
                                    items: [
                                        102,
                                    ],
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 104,
                                    items: [
                                        105,
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
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 107,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 108,
                                },
                                Prefix {
                                    opr: Tilde,
                                    opd: 109,
                                },
                                Binary {
                                    lopd: 106,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 110,
                                },
                                Binary {
                                    lopd: 103,
                                    opr: Assign,
                                    ropd: 111,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 341,
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
                                    opd: 115,
                                },
                                Binary {
                                    lopd: 114,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 116,
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
                                    opd: 119,
                                },
                                Binary {
                                    lopd: 118,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 120,
                                },
                                Binary {
                                    lopd: 117,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 121,
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
                                    lopd: 123,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 124,
                                },
                                Binary {
                                    lopd: 122,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 125,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    opd: 128,
                                },
                                Binary {
                                    lopd: 127,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 129,
                                },
                                Binary {
                                    lopd: 126,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 130,
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
                                    lopd: 132,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 133,
                                },
                                Binary {
                                    lopd: 131,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 134,
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
                                MethodCall {
                                    self_argument: 136,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 137,
                                    opr: UnwrapOrComposeWithNot,
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
                                                value: 337,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 139,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            140,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 138,
                                    opr: Assign,
                                    ropd: 141,
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
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
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
                                FnCall {
                                    function: 144,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            145,
                                        ),
                                        Regular(
                                            146,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 143,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            147,
                                        ),
                                    ],
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
                                    opd: 150,
                                },
                                Binary {
                                    lopd: 149,
                                    opr: Assign,
                                    ropd: 151,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    opd: 154,
                                },
                                Binary {
                                    lopd: 153,
                                    opr: Assign,
                                    ropd: 155,
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
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 158,
                                },
                                Binary {
                                    lopd: 157,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 159,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    lopd: 161,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 162,
                                },
                                Binary {
                                    lopd: 160,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 163,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    lopd: 165,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 166,
                                },
                                Binary {
                                    lopd: 164,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 167,
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
                                MethodCall {
                                    self_argument: 169,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 170,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
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
                                FnCall {
                                    function: 172,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            173,
                                        ),
                                        Regular(
                                            174,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 171,
                                    opr: Assign,
                                    ropd: 175,
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
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 177,
                                    opr: Assign,
                                    ropd: 178,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
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
                                Binary {
                                    lopd: 180,
                                    opr: Assign,
                                    ropd: 181,
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
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 184,
                                },
                                Binary {
                                    lopd: 183,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 185,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    lopd: 187,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 188,
                                },
                                Binary {
                                    lopd: 186,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 189,
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
                                    lopd: 191,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 192,
                                },
                                Binary {
                                    lopd: 190,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 193,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    lopd: 195,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 196,
                                },
                                Binary {
                                    lopd: 194,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 197,
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
                                MethodCall {
                                    self_argument: 199,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 142,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 200,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
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
                                FnCall {
                                    function: 202,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            203,
                                        ),
                                        Regular(
                                            204,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 201,
                                    opr: Assign,
                                    ropd: 205,
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
                                    opd: 208,
                                },
                                Binary {
                                    lopd: 207,
                                    opr: Assign,
                                    ropd: 209,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    opd: 212,
                                },
                                Binary {
                                    lopd: 211,
                                    opr: Assign,
                                    ropd: 213,
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
                                AssociatedFn,
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 258,
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
                                FnCall {
                                    function: 216,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            217,
                                        ),
                                        Regular(
                                            218,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 215,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            219,
                                        ),
                                    ],
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
                                                value: 344,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 221,
                                    opr: Assign,
                                    ropd: 222,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
                                            },
                                        ),
                                    ),
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
                                Binary {
                                    lopd: 224,
                                    opr: Assign,
                                    ropd: 225,
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
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 227,
                                    opr: Assign,
                                    ropd: 228,
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
                                                value: 341,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 230,
                                    opr: Assign,
                                    ropd: 231,
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
                                    lopd: 233,
                                    opr: Assign,
                                    ropd: 234,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
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
                                    lopd: 236,
                                    opr: Assign,
                                    ropd: 237,
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
                                    opd: 240,
                                },
                                Binary {
                                    lopd: 239,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 241,
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
                                Suffix {
                                    opd: 243,
                                    opr: Incr,
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
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 246,
                                },
                                Binary {
                                    lopd: 245,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 247,
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
                                    lopd: 249,
                                    opr: Comparison(
                                        Eq,
                                    ),
                                    ropd: 250,
                                },
                                Binary {
                                    lopd: 248,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 251,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 344,
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
                                    lopd: 253,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 254,
                                },
                                Binary {
                                    lopd: 252,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 255,
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
                                MethodCall {
                                    self_argument: 257,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 143,
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
                                                    value: 46,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 284,
                                            },
                                        ),
                                    ),
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
                                FnCall {
                                    function: 260,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            261,
                                        ),
                                        Regular(
                                            262,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 259,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 139,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            263,
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
                                        50..55,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 10,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 14,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 17,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 20,
                                },
                                Eval {
                                    expr_idx: 40,
                                },
                                Eval {
                                    expr_idx: 142,
                                },
                                Eval {
                                    expr_idx: 148,
                                },
                                Eval {
                                    expr_idx: 152,
                                },
                                Eval {
                                    expr_idx: 156,
                                },
                                Eval {
                                    expr_idx: 176,
                                },
                                Eval {
                                    expr_idx: 179,
                                },
                                Eval {
                                    expr_idx: 182,
                                },
                                Eval {
                                    expr_idx: 206,
                                },
                                Eval {
                                    expr_idx: 210,
                                },
                                Eval {
                                    expr_idx: 214,
                                },
                                Eval {
                                    expr_idx: 220,
                                },
                                Eval {
                                    expr_idx: 223,
                                },
                                Eval {
                                    expr_idx: 226,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 135,
                                        stmts: ArenaIdxRange(
                                            5..9,
                                        ),
                                    },
                                    elif_branches: [
                                        HirEagerElifBranch {
                                            condition: 168,
                                            stmts: ArenaIdxRange(
                                                9..12,
                                            ),
                                        },
                                        HirEagerElifBranch {
                                            condition: 198,
                                            stmts: ArenaIdxRange(
                                                12..15,
                                            ),
                                        },
                                    ],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                15..18,
                                            ),
                                        },
                                    ),
                                },
                                Eval {
                                    expr_idx: 229,
                                },
                                Eval {
                                    expr_idx: 232,
                                },
                                Eval {
                                    expr_idx: 235,
                                },
                                Eval {
                                    expr_idx: 244,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 21,
                                        ty: None,
                                    },
                                    initial_value: 96,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 22,
                                        ty: None,
                                    },
                                    initial_value: 100,
                                },
                                Eval {
                                    expr_idx: 112,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 113,
                                        stmts: ArenaIdxRange(
                                            18..22,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Match,
                                Eval {
                                    expr_idx: 238,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 242,
                                        stmts: ArenaIdxRange(
                                            22..23,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 258,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    initial_value: 46,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 7,
                                        ty: None,
                                    },
                                    initial_value: 47,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 8,
                                        ty: None,
                                    },
                                    initial_value: 51,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 9,
                                        ty: None,
                                    },
                                    initial_value: 57,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 10,
                                        ty: None,
                                    },
                                    initial_value: 61,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 11,
                                        ty: None,
                                    },
                                    initial_value: 66,
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
                                    initial_value: 74,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 19,
                                        ty: None,
                                    },
                                    initial_value: 76,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 20,
                                        ty: None,
                                    },
                                    initial_value: 78,
                                },
                                DoWhile {
                                    condition: 90,
                                    block: ArenaIdxRange(
                                        23..30,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 256,
                                        stmts: ArenaIdxRange(
                                            30..31,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 264,
                                },
                                While {
                                    condition: 45,
                                    stmts: ArenaIdxRange(
                                        31..49,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 36,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    initial_value: 0,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 2,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 258,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    3,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    4,
                                                ),
                                                kind: UpperClosed,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        0..5,
                                    ),
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 124,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    41,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    42,
                                                ),
                                                kind: UpperClosed,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        49..50,
                                    ),
                                },
                                Return {
                                    result: 265,
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
                                                value: 331,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 333,
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
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 270,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 314,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 315,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 324,
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
                            ],
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::raw_contour`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                            value: 29,
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
                                        value: 29,
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
                                    value: 27,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        3,
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
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                EmptyHtmlTag {
                                    function_ident: Ident(
                                        Coword(
                                            Id {
                                                value: 286,
                                            },
                                        ),
                                    ),
                                    arguments: [
                                        HirEagerHtmlArgumentExpr {
                                            property_ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
                                                    },
                                                ),
                                            ),
                                            expr: 1,
                                        },
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
                                    expr_idx: 2,
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
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::raw_contour`,
                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 29,
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
                                    value: 52,
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
                    body: Some(
                        4,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                AssociatedFn,
                                SelfValue,
                                Literal(
                                    F32(
                                        NotNan(
                                            1.4,
                                        ),
                                    ),
                                ),
                                FnCall {
                                    function: 0,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            1,
                                        ),
                                        Regular(
                                            2,
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
                                    expr_idx: 3,
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
                                    value: 49,
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
                    body: Some(
                        53,
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
                                                value: 260,
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
                                    owner: 1,
                                    items: [
                                        2,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 4,
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
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 6,
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
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 10,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 12,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 13,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                SelfValue,
                                Field {
                                    owner: 15,
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
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 16,
                                    items: [
                                        17,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
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
                                Field {
                                    owner: 21,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 20,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            22,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 19,
                                    opr: Assign,
                                    ropd: 23,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 293,
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
                                Field {
                                    owner: 27,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 26,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 58,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            28,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 25,
                                    opr: Assign,
                                    ropd: 29,
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
                                                value: 296,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 33,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 32,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 63,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            34,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 31,
                                    opr: Assign,
                                    ropd: 35,
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
                                                value: 296,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 39,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 38,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 58,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            40,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 37,
                                    opr: Assign,
                                    ropd: 41,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 53,
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
                                                    value: 52,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
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
                                FnCall {
                                    function: 44,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            45,
                                        ),
                                        Regular(
                                            46,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 52,
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
                                    function: 48,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            49,
                                        ),
                                        Regular(
                                            50,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 43,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            47,
                                        ),
                                        Regular(
                                            51,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        5..12,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 18,
                                },
                                Eval {
                                    expr_idx: 24,
                                },
                                Eval {
                                    expr_idx: 30,
                                },
                                Eval {
                                    expr_idx: 36,
                                },
                                Eval {
                                    expr_idx: 42,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 5,
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
                                    initial_value: 9,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 11,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 258,
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
                                                    14,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        0..5,
                                    ),
                                },
                                Return {
                                    result: 52,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 291,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
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
                            ],
                        },
                    },
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
                                    value: 50,
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
                    body: Some(
                        9,
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
                                                value: 284,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 254,
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
                                    owner: 2,
                                    items: [
                                        3,
                                    ],
                                },
                                Field {
                                    owner: 4,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 289,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 289,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 5,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 298,
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
                                    expr_idx: 8,
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
                    body: Some(
                        58,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
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
                                SelfValue,
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 3,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                SelfValue,
                                Field {
                                    owner: 5,
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
                                                value: 258,
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
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 8,
                                },
                                Index {
                                    owner: 6,
                                    items: [
                                        9,
                                    ],
                                },
                                SelfValue,
                                Field {
                                    owner: 11,
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
                                                value: 258,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 12,
                                    items: [
                                        13,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 16,
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
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 18,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 17,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 19,
                                },
                                MethodCall {
                                    self_argument: 20,
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
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 24,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 23,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 25,
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
                                Binary {
                                    lopd: 21,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 27,
                                },
                                Binary {
                                    lopd: 15,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 28,
                                },
                                SelfValue,
                                Field {
                                    owner: 30,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 32,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 33,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                Binary {
                                    lopd: 34,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 35,
                                },
                                Index {
                                    owner: 31,
                                    items: [
                                        36,
                                    ],
                                },
                                SelfValue,
                                Field {
                                    owner: 38,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
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
                                    owner: 39,
                                    items: [
                                        40,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 43,
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
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 45,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 44,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 46,
                                },
                                MethodCall {
                                    self_argument: 47,
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
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 49,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 51,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 50,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 52,
                                },
                                MethodCall {
                                    self_argument: 53,
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
                                Binary {
                                    lopd: 48,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 54,
                                },
                                Binary {
                                    lopd: 42,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 55,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        3..9,
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
                                    initial_value: 10,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 14,
                                },
                                Eval {
                                    expr_idx: 29,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 0,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 258,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    1,
                                                ),
                                                kind: LowerOpen,
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
                                        0..3,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 37,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 41,
                                },
                                Eval {
                                    expr_idx: 56,
                                },
                                Return {
                                    result: 57,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 154,
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
                                    value: 29,
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
                                        value: 29,
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
                                    value: 47,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        18,
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
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 138,
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
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 5,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    ropd: 6,
                                },
                                Index {
                                    owner: 4,
                                    items: [
                                        7,
                                    ],
                                },
                                SelfValue,
                                Field {
                                    owner: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 11,
                                    opr: Closed(
                                        RemEuclid,
                                    ),
                                    ropd: 12,
                                },
                                Index {
                                    owner: 10,
                                    items: [
                                        13,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 303,
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
                                    initial_value: 2,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 8,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 14,
                                },
                                Eval {
                                    expr_idx: 17,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 303,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 304,
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