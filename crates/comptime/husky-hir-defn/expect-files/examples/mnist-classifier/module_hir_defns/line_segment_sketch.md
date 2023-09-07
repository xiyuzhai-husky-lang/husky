[
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::line_segment_sketch::concave_component`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::line_segment_sketch::convex_component`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::line_segment_sketch::convexity`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::line_segment_sketch::line_segment`,
            },
        },
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `points`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 61,
                                        },
                                    ),
                                ),
                            },
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `contour`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `strokes`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 62,
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
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        50,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 1,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 3,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 5,
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
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 6,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 8,
                                },
                                Binary {
                                    lopd: 4,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 9,
                                },
                                MethodCall {
                                    self_argument: 10,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 73,
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
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 12,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 13,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 15,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 16,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 18,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 19,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 21,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 22,
                                },
                                Binary {
                                    lopd: 20,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 23,
                                },
                                MethodCall {
                                    self_argument: 24,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 73,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 17,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 25,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 381,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 28,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 27,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 29,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 30,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 31,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 381,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Minus,
                                    opd: 33,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 35,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 34,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 36,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 37,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 38,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 41,
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
                                                value: 382,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 42,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 43,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 45,
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
                                                value: 383,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 46,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 47,
                                },
                                FnCall {
                                    function: 40,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            44,
                                        ),
                                        Regular(
                                            48,
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
                                    initial_value: 11,
                                },
                                Assert {
                                    condition: 14,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 26,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 32,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 39,
                                },
                                Eval {
                                    expr_idx: 49,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 381,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 382,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 383,
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
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        50,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 1,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 3,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 5,
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
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 6,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 8,
                                },
                                Binary {
                                    lopd: 4,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 9,
                                },
                                MethodCall {
                                    self_argument: 10,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 73,
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
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 12,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 13,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 15,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 16,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 18,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 19,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 21,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 22,
                                },
                                Binary {
                                    lopd: 20,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 23,
                                },
                                MethodCall {
                                    self_argument: 24,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 73,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Binary {
                                    lopd: 17,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 25,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 381,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Minus,
                                    opd: 27,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 29,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 28,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 30,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 31,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 32,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 381,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 35,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 34,
                                    opr: Closed(
                                        Mul,
                                    ),
                                    ropd: 36,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 37,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 38,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 41,
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
                                                value: 382,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 42,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 43,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 380,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 45,
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
                                                value: 383,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 46,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 47,
                                },
                                FnCall {
                                    function: 40,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            44,
                                        ),
                                        Regular(
                                            48,
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
                                    initial_value: 11,
                                },
                                Assert {
                                    condition: 14,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 1,
                                        ty: None,
                                    },
                                    initial_value: 26,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 33,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 39,
                                },
                                Eval {
                                    expr_idx: 49,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 381,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 382,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 383,
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
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        114,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
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
                                                value: 249,
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
                                                value: 147,
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
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 4,
                                },
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
                                    item_groups: [
                                        Regular(
                                            2,
                                        ),
                                        Regular(
                                            5,
                                        ),
                                    ],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 8,
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
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 13,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 14,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 16,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 17,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 18,
                                },
                                Binary {
                                    lopd: 15,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 19,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Suffix {
                                    opd: 21,
                                    opr: Incr,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
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
                                                value: 147,
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
                                    lopd: 26,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 27,
                                },
                                MethodCall {
                                    self_argument: 24,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            25,
                                        ),
                                        Regular(
                                            28,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 23,
                                    opr: Assign,
                                    ropd: 29,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 31,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 32,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 33,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 36,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            37,
                                        ),
                                        Regular(
                                            38,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 40,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            41,
                                        ),
                                        Regular(
                                            42,
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
                                CurrentSymbol {
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
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 45,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 46,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 48,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            49,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 50,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 51,
                                },
                                Binary {
                                    lopd: 47,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 52,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 54,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            55,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 56,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 57,
                                },
                                Binary {
                                    lopd: 53,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 58,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 60,
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
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 63,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 64,
                                },
                                Binary {
                                    lopd: 62,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 65,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 67,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 68,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 70,
                                    opr: Assign,
                                    ropd: 71,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 73,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 74,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 76,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            77,
                                        ),
                                        Regular(
                                            78,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 80,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            81,
                                        ),
                                        Regular(
                                            82,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 392,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 84,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            85,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 86,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 87,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 392,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 89,
                                    opr: Assign,
                                    ropd: 90,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 393,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 92,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            93,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 94,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 95,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 393,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 97,
                                    opr: Assign,
                                    ropd: 98,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Suffix {
                                    opd: 100,
                                    opr: Incr,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
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
                                                value: 147,
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
                                    lopd: 105,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 106,
                                },
                                MethodCall {
                                    self_argument: 103,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            104,
                                        ),
                                        Regular(
                                            107,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 102,
                                    opr: Assign,
                                    ropd: 108,
                                },
                                CurrentSymbol {
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
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 110,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 111,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        16..28,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 22,
                                },
                                Eval {
                                    expr_idx: 30,
                                },
                                Return {
                                    result: 35,
                                },
                                Break,
                                Eval {
                                    expr_idx: 72,
                                },
                                Eval {
                                    expr_idx: 91,
                                },
                                Eval {
                                    expr_idx: 99,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 8,
                                        ty: None,
                                    },
                                    initial_value: 79,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 9,
                                        ty: None,
                                    },
                                    initial_value: 83,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 88,
                                        stmts: ArenaIdxRange(
                                            5..6,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 96,
                                        stmts: ArenaIdxRange(
                                            6..7,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 7,
                                        ty: None,
                                    },
                                    initial_value: 61,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 66,
                                        stmts: ArenaIdxRange(
                                            3..4,
                                        ),
                                    },
                                    elif_branches: [
                                        HirEagerElifBranch {
                                            condition: 69,
                                            stmts: ArenaIdxRange(
                                                4..5,
                                            ),
                                        },
                                    ],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 75,
                                        stmts: ArenaIdxRange(
                                            7..11,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 101,
                                },
                                Eval {
                                    expr_idx: 109,
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
                                    initial_value: 6,
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
                                    initial_value: 12,
                                },
                                While {
                                    condition: 20,
                                    stmts: ArenaIdxRange(
                                        0..2,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 34,
                                        stmts: ArenaIdxRange(
                                            2..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 39,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 43,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: None,
                                    },
                                    initial_value: 44,
                                },
                                While {
                                    condition: 59,
                                    stmts: ArenaIdxRange(
                                        11..16,
                                    ),
                                },
                                Assert {
                                    condition: 112,
                                },
                                Return {
                                    result: 113,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
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
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 392,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 393,
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
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        123,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
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
                                                value: 249,
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
                                                value: 146,
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
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 4,
                                },
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
                                    item_groups: [
                                        Regular(
                                            2,
                                        ),
                                        Regular(
                                            5,
                                        ),
                                    ],
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
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
                                Binary {
                                    lopd: 7,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 10,
                                },
                                CurrentSymbol {
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
                                                value: 397,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 12,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 13,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 396,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 15,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 16,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 17,
                                },
                                Binary {
                                    lopd: 14,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 18,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                Suffix {
                                    opd: 20,
                                    opr: Decr,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 396,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
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
                                                value: 146,
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
                                    lopd: 25,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 26,
                                },
                                MethodCall {
                                    self_argument: 23,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            24,
                                        ),
                                        Regular(
                                            27,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 22,
                                    opr: Assign,
                                    ropd: 28,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 396,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 30,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 31,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 32,
                                },
                                CurrentSymbol {
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
                                                value: 395,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 34,
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
                                            35,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 396,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 37,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            38,
                                        ),
                                        Regular(
                                            39,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 396,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 41,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            42,
                                        ),
                                        Regular(
                                            43,
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
                                CurrentSymbol {
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
                                                value: 397,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 46,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 47,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
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
                                                value: 146,
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
                                    lopd: 51,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 52,
                                },
                                MethodCall {
                                    self_argument: 49,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 300,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            50,
                                        ),
                                        Regular(
                                            53,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 55,
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
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 58,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 59,
                                },
                                Binary {
                                    lopd: 57,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 60,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 62,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 63,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 65,
                                    opr: Assign,
                                    ropd: 66,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 68,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 69,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 18,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 71,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            72,
                                        ),
                                        Regular(
                                            73,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 19,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 75,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            76,
                                        ),
                                        Regular(
                                            77,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 392,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 79,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            80,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 81,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 82,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 392,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 84,
                                    opr: Assign,
                                    ropd: 85,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 393,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 87,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            88,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 89,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 90,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 393,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 92,
                                    opr: Assign,
                                    ropd: 93,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 95,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            96,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 97,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 98,
                                },
                                CurrentSymbol {
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
                                                value: 395,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 100,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 101,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 103,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            104,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 105,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 106,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 108,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 359,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            109,
                                        ),
                                    ],
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 110,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 111,
                                },
                                Binary {
                                    lopd: 107,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 112,
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 113,
                                },
                                Binary {
                                    lopd: 102,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 114,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                Suffix {
                                    opd: 116,
                                    opr: Decr,
                                },
                                CurrentSymbol {
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
                                                value: 395,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 118,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 119,
                                },
                                CurrentSymbol {
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
                                                value: 395,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        22..32,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 21,
                                },
                                Eval {
                                    expr_idx: 29,
                                },
                                Return {
                                    result: 36,
                                },
                                Break,
                                Eval {
                                    expr_idx: 67,
                                },
                                Eval {
                                    expr_idx: 86,
                                },
                                Eval {
                                    expr_idx: 94,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 8,
                                        ty: None,
                                    },
                                    initial_value: 74,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 9,
                                        ty: None,
                                    },
                                    initial_value: 78,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 83,
                                        stmts: ArenaIdxRange(
                                            5..6,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 91,
                                        stmts: ArenaIdxRange(
                                            6..7,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Break,
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 115,
                                        stmts: ArenaIdxRange(
                                            11..12,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 117,
                                },
                                Break,
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: None,
                                    },
                                    initial_value: 54,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 7,
                                        ty: None,
                                    },
                                    initial_value: 56,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 61,
                                        stmts: ArenaIdxRange(
                                            3..4,
                                        ),
                                    },
                                    elif_branches: [
                                        HirEagerElifBranch {
                                            condition: 64,
                                            stmts: ArenaIdxRange(
                                                4..5,
                                            ),
                                        },
                                    ],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 70,
                                        stmts: ArenaIdxRange(
                                            7..11,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 99,
                                        stmts: ArenaIdxRange(
                                            12..14,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                14..15,
                                            ),
                                        },
                                    ),
                                },
                                Return {
                                    result: 121,
                                },
                                Return {
                                    result: 122,
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
                                    initial_value: 6,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 11,
                                },
                                While {
                                    condition: 19,
                                    stmts: ArenaIdxRange(
                                        0..2,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 33,
                                        stmts: ArenaIdxRange(
                                            2..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 40,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 44,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 45,
                                },
                                While {
                                    condition: 48,
                                    stmts: ArenaIdxRange(
                                        15..20,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 120,
                                        stmts: ArenaIdxRange(
                                            20..21,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                21..22,
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
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 396,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 397,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 388,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 389,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 390,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 391,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 392,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 393,
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
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        191,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                List {
                                    items: [],
                                },
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
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
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 6,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 7,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 20,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
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
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 10,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            11,
                                        ),
                                        Regular(
                                            12,
                                        ),
                                        Regular(
                                            13,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 9,
                                    opr: Assign,
                                    ropd: 14,
                                },
                                AssociatedFn,
                                InheritedSymbol {
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
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 16,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            17,
                                        ),
                                        Regular(
                                            18,
                                        ),
                                        Regular(
                                            19,
                                        ),
                                    ],
                                },
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 22,
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
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 23,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 24,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 399,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 26,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 28,
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
                                    opd: 29,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                MethodCall {
                                    self_argument: 30,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 401,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 402,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 32,
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
                                            33,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 34,
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
                                Literal(
                                    F32(
                                        NotNan(
                                            0.01,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 35,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 36,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 401,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 402,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 38,
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
                                            39,
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
                                    lopd: 40,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 41,
                                },
                                Binary {
                                    lopd: 37,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 42,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 44,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 45,
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
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 47,
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
                                    opd: 48,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                AssociatedFn,
                                InheritedSymbol {
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
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 52,
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
                                    opd: 53,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 54,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 55,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
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
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 50,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            51,
                                        ),
                                        Regular(
                                            56,
                                        ),
                                        Regular(
                                            57,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 49,
                                    opr: Assign,
                                    ropd: 58,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 400,
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
                                    lopd: 60,
                                    opr: Assign,
                                    ropd: 61,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 400,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 21,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
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
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
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
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 65,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            66,
                                        ),
                                        Regular(
                                            67,
                                        ),
                                        Regular(
                                            68,
                                        ),
                                        Regular(
                                            69,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 64,
                                    opr: Assign,
                                    ropd: 70,
                                },
                                AssociatedFn,
                                InheritedSymbol {
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
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 72,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            73,
                                        ),
                                        Regular(
                                            74,
                                        ),
                                        Regular(
                                            75,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 77,
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
                                        0,
                                    ),
                                ),
                                Binary {
                                    lopd: 78,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 79,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 81,
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
                                    opd: 82,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 404,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 84,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 403,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 86,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 404,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 88,
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
                                                value: 403,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 90,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 89,
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
                                            91,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 405,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 93,
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
                                            94,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 95,
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
                                Literal(
                                    F32(
                                        NotNan(
                                            0.001,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 96,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 97,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 405,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 99,
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
                                            100,
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
                                    lopd: 101,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 102,
                                },
                                Binary {
                                    lopd: 98,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 103,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 406,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 105,
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
                                            106,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 107,
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
                                Literal(
                                    F32(
                                        NotNan(
                                            0.001,
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 108,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 109,
                                },
                                Binary {
                                    lopd: 104,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 110,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 406,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 112,
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
                                            113,
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
                                    lopd: 114,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 115,
                                },
                                Binary {
                                    lopd: 111,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 116,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 118,
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
                                Suffix {
                                    opd: 119,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 403,
                                            },
                                        ),
                                    ),
                                },
                                AssociatedFn,
                                InheritedSymbol {
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
                                                value: 404,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 124,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 125,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
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
                                                value: 403,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 127,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 128,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                FnCall {
                                    function: 122,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            123,
                                        ),
                                        Regular(
                                            126,
                                        ),
                                        Regular(
                                            129,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 121,
                                    opr: Assign,
                                    ropd: 130,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
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
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 134,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 135,
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
                                Binary {
                                    lopd: 133,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 136,
                                },
                                Binary {
                                    lopd: 132,
                                    opr: Assign,
                                    ropd: 137,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 403,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 139,
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
                                            140,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
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
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 142,
                                    opr: Assign,
                                    ropd: 143,
                                },
                                CurrentSymbol {
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
                                                value: 146,
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
                                    lopd: 146,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 147,
                                },
                                Binary {
                                    lopd: 145,
                                    opr: Assign,
                                    ropd: 148,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 150,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 151,
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
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 153,
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
                                    opd: 154,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 155,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 156,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
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
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 158,
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
                                    opd: 159,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 408,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 161,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 162,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
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
                                                value: 407,
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
                                    lopd: 164,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 165,
                                },
                                Binary {
                                    lopd: 163,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 166,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 168,
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
                                Suffix {
                                    opd: 169,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 171,
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
                                    opd: 172,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                AssociatedFn,
                                InheritedSymbol {
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
                                                value: 408,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 176,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 177,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
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
                                                value: 302,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 178,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 179,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 181,
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
                                    opd: 182,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 183,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 260,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 184,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
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
                                    lopd: 185,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 186,
                                },
                                FnCall {
                                    function: 174,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            175,
                                        ),
                                        Regular(
                                            180,
                                        ),
                                        Regular(
                                            187,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 173,
                                    opr: Assign,
                                    ropd: 188,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        27..37,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 8,
                                        ty: None,
                                    },
                                    initial_value: 46,
                                },
                                Eval {
                                    expr_idx: 59,
                                },
                                Eval {
                                    expr_idx: 62,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 6,
                                        ty: None,
                                    },
                                    initial_value: 27,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 7,
                                        ty: None,
                                    },
                                    initial_value: 31,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 43,
                                        stmts: ArenaIdxRange(
                                            0..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 14,
                                        ty: None,
                                    },
                                    initial_value: 120,
                                },
                                Eval {
                                    expr_idx: 131,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 10,
                                        ty: None,
                                    },
                                    initial_value: 83,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 11,
                                        ty: None,
                                    },
                                    initial_value: 85,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 12,
                                        ty: None,
                                    },
                                    initial_value: 87,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 13,
                                        ty: None,
                                    },
                                    initial_value: 92,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 117,
                                        stmts: ArenaIdxRange(
                                            6..8,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 138,
                                },
                                Eval {
                                    expr_idx: 71,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 9,
                                        ty: None,
                                    },
                                    initial_value: 76,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 80,
                                        stmts: ArenaIdxRange(
                                            8..13,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        HirEagerElseBranch {
                                            stmts: ArenaIdxRange(
                                                13..14,
                                            ),
                                        },
                                    ),
                                },
                                Eval {
                                    expr_idx: 141,
                                },
                                Eval {
                                    expr_idx: 15,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 20,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 21,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 25,
                                        stmts: ArenaIdxRange(
                                            3..6,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 63,
                                        stmts: ArenaIdxRange(
                                            14..18,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 144,
                                },
                                Eval {
                                    expr_idx: 149,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 18,
                                        ty: None,
                                    },
                                    initial_value: 170,
                                },
                                Eval {
                                    expr_idx: 189,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 62,
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
                                    initial_value: 1,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 2,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 5,
                                },
                                While {
                                    condition: 8,
                                    stmts: ArenaIdxRange(
                                        18..25,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 15,
                                        ty: None,
                                    },
                                    initial_value: 152,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 16,
                                        ty: None,
                                    },
                                    initial_value: 157,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 17,
                                        ty: None,
                                    },
                                    initial_value: 160,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 167,
                                        stmts: ArenaIdxRange(
                                            25..27,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 190,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 398,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 399,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 400,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 401,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 402,
                                            },
                                        ),
                                    ),
                                },
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
                                                value: 403,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 404,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 405,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 386,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 406,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 404,
                                            },
                                        ),
                                    ),
                                },
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
                                                value: 407,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 408,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 408,
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
                    module_path: `mnist_classifier::line_segment_sketch`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                            value: 54,
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
                            module_path: `mnist_classifier::line_segment_sketch`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                        value: 54,
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
                                EmptyHtmlTag {
                                    function_ident: Ident(
                                        Coword(
                                            Id {
                                                value: 371,
                                            },
                                        ),
                                    ),
                                    arguments: [
                                        HirEagerHtmlArgumentExpr {
                                            property_ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 146,
                                                    },
                                                ),
                                            ),
                                            expr: 1,
                                        },
                                        HirEagerHtmlArgumentExpr {
                                            property_ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 147,
                                                    },
                                                ),
                                            ),
                                            expr: 3,
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
    HirDefn::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 54,
                        },
                    ),
                ),
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::AssociatedFn(
                TypeAssociatedFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `new`,
                        item_kind: AssociatedFn,
                    },
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `new`,
                            item_kind: AssociatedFn,
                        },
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 30,
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
                                    value: 54,
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
                        12,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 372,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 305,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 0,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 1,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 4,
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
                                                value: 372,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 305,
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
                                        Add,
                                    ),
                                    ropd: 8,
                                },
                                MethodCall {
                                    self_argument: 5,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 145,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            6,
                                        ),
                                        Regular(
                                            9,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 3,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            10,
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
                                Assert {
                                    condition: 2,
                                },
                                Eval {
                                    expr_idx: 11,
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
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 54,
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
                                        value: 54,
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
                                                value: 305,
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
    HirDefn::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `mnist_classifier::line_segment_sketch`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                            value: 52,
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
                            module_path: `mnist_classifier::line_segment_sketch`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                        value: 52,
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
                                                value: 373,
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
                    module_path: `mnist_classifier::line_segment_sketch`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 52,
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
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `concave_components`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `concave_components`,
                            item_kind: MemoizedField,
                        },
                        return_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 57,
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
                                                    value: 23,
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
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                        55,
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
                                                value: 373,
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
                                    owner: 5,
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
                                    owner: 7,
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
                                    owner: 9,
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
                                    owner: 11,
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
                                    owner: 13,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 373,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 14,
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
                                    owner: 16,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 373,
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
                                    owner: 17,
                                    items: [
                                        18,
                                    ],
                                },
                                Field {
                                    owner: 19,
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
                                    owner: 23,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 22,
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
                                            24,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 21,
                                    opr: Assign,
                                    ropd: 25,
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
                                    owner: 29,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 273,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 28,
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
                                            30,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 27,
                                    opr: Assign,
                                    ropd: 31,
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
                                    owner: 35,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 34,
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
                                            36,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 33,
                                    opr: Assign,
                                    ropd: 37,
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
                                    owner: 41,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 40,
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
                                            42,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 39,
                                    opr: Assign,
                                    ropd: 43,
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
                                    function: 46,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            47,
                                        ),
                                        Regular(
                                            48,
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
                                    function: 50,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            51,
                                        ),
                                        Regular(
                                            52,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 45,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            49,
                                        ),
                                        Regular(
                                            53,
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
                                    initial_value: 20,
                                },
                                Eval {
                                    expr_idx: 26,
                                },
                                Eval {
                                    expr_idx: 32,
                                },
                                Eval {
                                    expr_idx: 38,
                                },
                                Eval {
                                    expr_idx: 44,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 4,
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
                                    initial_value: 8,
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
                                        0..5,
                                    ),
                                },
                                Return {
                                    result: 54,
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
            TypeItemHirDefn::AssociatedFn(
                TypeAssociatedFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `new`,
                        item_kind: AssociatedFn,
                    },
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `new`,
                            item_kind: AssociatedFn,
                        },
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        parenate_parameters: HirParenateParameters {
                            data: [
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 30,
                                            },
                                        ),
                                    ),
                                },
                                HirRitchieParameter {
                                    contract: None,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                },
                            ],
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
                        7,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 56,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 22,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 249,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 377,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 2,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            3,
                                        ),
                                        Regular(
                                            4,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 0,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            1,
                                        ),
                                        Regular(
                                            5,
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
]