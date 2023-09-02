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
                                            value: 32,
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
                                                value: 250,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 251,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 152,
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
                                    lopd: 2,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 3,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 152,
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
                                    lopd: 5,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 6,
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
                                    condition: 4,
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
                                                value: 152,
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
                                            value: 33,
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
                    body: Some(
                        42,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
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
                                                value: 274,
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
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 3,
                                },
                                Binary {
                                    lopd: 1,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 4,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
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
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 7,
                                },
                                Binary {
                                    lopd: 5,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 8,
                                },
                                Binary {
                                    lopd: 0,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 9,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
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
                                                value: 275,
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
                                    lopd: 13,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 14,
                                },
                                Binary {
                                    lopd: 12,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 15,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
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
                                    lopd: 17,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 18,
                                },
                                Binary {
                                    lopd: 16,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 19,
                                },
                                Binary {
                                    lopd: 11,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 20,
                                },
                                CurrentSymbol {
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
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 22,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 23,
                                },
                                CurrentSymbol {
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
                                                value: 276,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 25,
                                    opr: Assign,
                                    ropd: 26,
                                },
                                CurrentSymbol {
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
                                                value: 51,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
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
                                                value: 275,
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
                                    lopd: 31,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 32,
                                },
                                Binary {
                                    lopd: 30,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 33,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
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
                                    lopd: 35,
                                    opr: Shift(
                                        Shr,
                                    ),
                                    ropd: 36,
                                },
                                Binary {
                                    lopd: 34,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 37,
                                },
                                Binary {
                                    lopd: 29,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 38,
                                },
                                Binary {
                                    lopd: 28,
                                    opr: Assign,
                                    ropd: 39,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        2..6,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 27,
                                },
                                Eval {
                                    expr_idx: 40,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 10,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 21,
                                },
                                While {
                                    condition: 24,
                                    stmts: ArenaIdxRange(
                                        0..2,
                                    ),
                                },
                                Return {
                                    result: 41,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 276,
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
                    path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        105,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                List {
                                    items: [],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 278,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
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
                                                value: 279,
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
                                    owner: 4,
                                    items: [
                                        5,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 279,
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
                                    owner: 7,
                                    items: [
                                        8,
                                    ],
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
                                MethodCall {
                                    self_argument: 10,
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
                                AssociatedFn,
                                FnCall {
                                    function: 12,
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                CurrentSymbol {
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
                                                value: 271,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 14,
                                    items: [
                                        15,
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
                                                value: 51,
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
                                                value: 280,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 19,
                                    opr: Shift(
                                        Shl,
                                    ),
                                    ropd: 20,
                                },
                                FnCall {
                                    function: 17,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            18,
                                        ),
                                        Regular(
                                            21,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 16,
                                    opr: Assign,
                                    ropd: 22,
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
                                                value: 282,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 25,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 282,
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
                                    lopd: 27,
                                    opr: Assign,
                                    ropd: 28,
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
                                CurrentSymbol {
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
                                                value: 259,
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
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 33,
                                },
                                Index {
                                    owner: 31,
                                    items: [
                                        34,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 283,
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
                                                value: 278,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
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
                                    lopd: 39,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 40,
                                },
                                Index {
                                    owner: 38,
                                    items: [
                                        41,
                                    ],
                                },
                                CurrentSymbol {
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
                                                value: 259,
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
                                FnCall {
                                    function: 37,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            42,
                                        ),
                                        Regular(
                                            45,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 36,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 46,
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
                                Prefix {
                                    opr: Not,
                                    opd: 48,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 283,
                                            },
                                        ),
                                    ),
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
                                Binary {
                                    lopd: 50,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 51,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 282,
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
                                    lopd: 53,
                                    opr: Assign,
                                    ropd: 54,
                                },
                                CurrentSymbol {
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
                                                value: 259,
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
                                    lopd: 57,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 58,
                                },
                                Index {
                                    owner: 56,
                                    items: [
                                        59,
                                    ],
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
                                Binary {
                                    lopd: 60,
                                    opr: Assign,
                                    ropd: 61,
                                },
                                CurrentSymbol {
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 63,
                                    items: [
                                        64,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 283,
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
                                                value: 278,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 68,
                                    items: [
                                        69,
                                    ],
                                },
                                CurrentSymbol {
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
                                                value: 259,
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
                                    lopd: 72,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 73,
                                },
                                Index {
                                    owner: 71,
                                    items: [
                                        74,
                                    ],
                                },
                                FnCall {
                                    function: 67,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            70,
                                        ),
                                        Regular(
                                            75,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 66,
                                    opr: Closed(
                                        BitOr,
                                    ),
                                    ropd: 76,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 283,
                                            },
                                        ),
                                    ),
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
                                Binary {
                                    lopd: 78,
                                    opr: Comparison(
                                        Neq,
                                    ),
                                    ropd: 79,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 282,
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
                                    lopd: 81,
                                    opr: Assign,
                                    ropd: 82,
                                },
                                CurrentSymbol {
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 84,
                                    items: [
                                        85,
                                    ],
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
                                Binary {
                                    lopd: 86,
                                    opr: Assign,
                                    ropd: 87,
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
                                Literal(
                                    I32(
                                        30,
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 279,
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
                                    owner: 91,
                                    items: [
                                        92,
                                    ],
                                },
                                CurrentSymbol {
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
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 94,
                                    items: [
                                        95,
                                    ],
                                },
                                Prefix {
                                    opr: Tilde,
                                    opd: 96,
                                },
                                Binary {
                                    lopd: 93,
                                    opr: AssignClosed(
                                        BitAnd,
                                    ),
                                    ropd: 97,
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
                                                value: 253,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 100,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            101,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 99,
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
                                            102,
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
                                        26..30,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Break,
                                Eval {
                                    expr_idx: 55,
                                },
                                Eval {
                                    expr_idx: 62,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 7,
                                        ty: None,
                                    },
                                    initial_value: 35,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 8,
                                        ty: None,
                                    },
                                    initial_value: 47,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 49,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 52,
                                        stmts: ArenaIdxRange(
                                            1..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 83,
                                },
                                Eval {
                                    expr_idx: 88,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 9,
                                        ty: None,
                                    },
                                    initial_value: 65,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 10,
                                        ty: None,
                                    },
                                    initial_value: 77,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 80,
                                        stmts: ArenaIdxRange(
                                            7..9,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 29,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: None,
                                    },
                                    initial_value: 30,
                                },
                                ForExt {
                                    particulars: HirEagerForExtParticulars,
                                    block: ArenaIdxRange(
                                        3..7,
                                    ),
                                },
                                ForExt {
                                    particulars: HirEagerForExtParticulars,
                                    block: ArenaIdxRange(
                                        9..12,
                                    ),
                                },
                                Eval {
                                    expr_idx: 98,
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
                                    initial_value: 11,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 13,
                                },
                                Eval {
                                    expr_idx: 23,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 24,
                                },
                                While {
                                    condition: 26,
                                    stmts: ArenaIdxRange(
                                        12..16,
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
                                                    89,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    90,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        16..17,
                                    ),
                                },
                                Eval {
                                    expr_idx: 103,
                                },
                                While {
                                    condition: 6,
                                    stmts: ArenaIdxRange(
                                        17..25,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 35,
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
                                                    value: 271,
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
                                        25..26,
                                    ),
                                },
                                Return {
                                    result: 104,
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
                                                value: 279,
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
                                                value: 280,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 253,
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 283,
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
                                                value: 283,
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
                            value: 34,
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
                                        value: 34,
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
                                                value: 253,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 156,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
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
                            value: 34,
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
                                    value: 36,
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
                        3,
                    ),
                    hir_expr_region: HirEagerExprRegion {
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
                                SelfValue,
                                FnCall {
                                    function: 0,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            1,
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
                                    value: 37,
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
                        20,
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
                                                value: 255,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 144,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                List {
                                    items: [],
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
                                    self_argument: 4,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 149,
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
                                                value: 247,
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
                                    self_argument: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 149,
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
                                MethodCall {
                                    self_argument: 7,
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
                                            10,
                                        ),
                                    ],
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 255,
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
                                    self_argument: 13,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 149,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            14,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 12,
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
                                            15,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
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
                                    function: 17,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            18,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        0..6,
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
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 32,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    initial_value: 3,
                                },
                                Eval {
                                    expr_idx: 6,
                                },
                                Eval {
                                    expr_idx: 11,
                                },
                                Eval {
                                    expr_idx: 16,
                                },
                                Return {
                                    result: 19,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
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
                                                value: 247,
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
                    body: Some(
                        20,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 255,
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
                                                value: 255,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 4,
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 6,
                                    items: [
                                        7,
                                    ],
                                },
                                Field {
                                    owner: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 261,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 9,
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
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 11,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 12,
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
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 14,
                                    opr: Assign,
                                    ropd: 15,
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
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 17,
                                    opr: As,
                                    ropd: 18,
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
                                    expr_idx: 16,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 10,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 13,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
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
                                                    value: 259,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    3,
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
                                        1..3,
                                    ),
                                },
                                Return {
                                    result: 19,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
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
                                                value: 255,
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
                    body: Some(
                        15,
                    ),
                    hir_expr_region: HirEagerExprRegion {
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
                                                value: 263,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 263,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 5,
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 6,
                                    items: [
                                        7,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 127,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                MethodCall {
                                    self_argument: 4,
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
                                            9,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 3,
                                    opr: Assign,
                                    ropd: 10,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 263,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 12,
                                    opr: As,
                                    ropd: 13,
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
                                Eval {
                                    expr_idx: 11,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
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
                                    initial_value: 0,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 259,
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
                                                    2,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                                Return {
                                    result: 14,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 263,
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
                    body: Some(
                        13,
                    ),
                    hir_expr_region: HirEagerExprRegion {
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
                                                value: 264,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                Field {
                                    owner: 4,
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 5,
                                    items: [
                                        6,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 127,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 3,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 8,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 264,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 10,
                                    opr: As,
                                    ropd: 11,
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
                                Eval {
                                    expr_idx: 9,
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
                                                    value: 259,
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
                                                    2,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                                Return {
                                    result: 12,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 264,
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
                    body: Some(
                        49,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                SelfValue,
                                Field {
                                    owner: 1,
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
                                                value: 242,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 2,
                                    items: [
                                        3,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 242,
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
                                    lopd: 5,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 6,
                                },
                                SelfValue,
                                Field {
                                    owner: 8,
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
                                                value: 243,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 9,
                                    items: [
                                        10,
                                    ],
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 11,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 243,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 242,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 13,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 14,
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
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                                Binary {
                                    lopd: 16,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 17,
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
                                                value: 242,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 242,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 267,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 21,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 22,
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
                                SelfValue,
                                Field {
                                    owner: 25,
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
                                                value: 268,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 26,
                                    items: [
                                        27,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 28,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 126,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 24,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 29,
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
                                                value: 243,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 243,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 267,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 33,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 34,
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
                                SelfValue,
                                Field {
                                    owner: 37,
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
                                                value: 269,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 38,
                                    items: [
                                        39,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 40,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 126,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 36,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 41,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 43,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 242,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 243,
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
                                                value: 245,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 43,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            44,
                                        ),
                                        Regular(
                                            45,
                                        ),
                                        Regular(
                                            46,
                                        ),
                                        Regular(
                                            47,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        6..17,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Break,
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
                                Break,
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 12,
                                        stmts: ArenaIdxRange(
                                            2..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 30,
                                },
                                Eval {
                                    expr_idx: 42,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 0,
                                },
                                ForExt {
                                    particulars: HirEagerForExtParticulars,
                                    block: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 7,
                                },
                                ForExt {
                                    particulars: HirEagerForExtParticulars,
                                    block: ArenaIdxRange(
                                        3..4,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 15,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 18,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 19,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 268,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    20,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    23,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        4..5,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 31,
                                },
                                ForBetween {
                                    particulars: HirEagerForBetweenParticulars {
                                        frame_var_ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 269,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    32,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    35,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            step: Constant(
                                                -1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        5..6,
                                    ),
                                },
                                Return {
                                    result: 48,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 242,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 243,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 266,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 267,
                                            },
                                        ),
                                    ),
                                },
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
                                                value: 265,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 244,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 2,
                                    opr: As,
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
                                                value: 265,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 245,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 2,
                                    opr: As,
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
                                    value: 34,
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
                                        value: 34,
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
                    body: Some(
                        23,
                    ),
                    hir_expr_region: HirEagerExprRegion {
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
                                                value: 124,
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
                                    lopd: 1,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 2,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                SelfValue,
                                Field {
                                    owner: 5,
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 6,
                                    items: [
                                        7,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 10,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 11,
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
                                SelfValue,
                                Field {
                                    owner: 14,
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
                                                value: 271,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 15,
                                    items: [
                                        16,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 17,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 127,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 13,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 18,
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
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 20,
                                    opr: As,
                                    ropd: 21,
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
                                Break,
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 8,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 19,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 0,
                                },
                                Assert {
                                    condition: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 4,
                                },
                                ForExt {
                                    particulars: HirEagerForExtParticulars,
                                    block: ArenaIdxRange(
                                        1..2,
                                    ),
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
                                                    9,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    12,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                                Return {
                                    result: 22,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
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
                                                value: 259,
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
                                    value: 34,
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
                                        value: 34,
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
                    body: Some(
                        23,
                    ),
                    hir_expr_region: HirEagerExprRegion {
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
                                                value: 124,
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
                                    lopd: 1,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 2,
                                },
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                SelfValue,
                                Field {
                                    owner: 5,
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
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 6,
                                    items: [
                                        7,
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 259,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 124,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 10,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 11,
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
                                SelfValue,
                                Field {
                                    owner: 14,
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
                                                value: 271,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 15,
                                    items: [
                                        16,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 17,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 128,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 13,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 18,
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
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 20,
                                    opr: As,
                                    ropd: 21,
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
                                Break,
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 8,
                                        stmts: ArenaIdxRange(
                                            0..1,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 19,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 0,
                                },
                                Assert {
                                    condition: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 4,
                                },
                                ForExt {
                                    particulars: HirEagerForExtParticulars,
                                    block: ArenaIdxRange(
                                        1..2,
                                    ),
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
                                                    9,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    12,
                                                ),
                                                kind: UpperOpen,
                                            },
                                            step: Constant(
                                                1,
                                            ),
                                        },
                                    },
                                    block: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                                Return {
                                    result: 22,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
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
                                                value: 259,
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