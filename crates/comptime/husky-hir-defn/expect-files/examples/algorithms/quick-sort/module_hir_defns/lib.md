[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
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
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 152,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
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
                                    USize(
                                        TermUSizeLiteral(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 5,
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
                                                    value: 18,
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
                                    initial_value: 1,
                                },
                                Eval {
                                    expr_idx: 10,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                    },
                    body: Some(
                        22,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 192,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 0,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 1,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 3,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 192,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 3,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            4,
                                        ),
                                        Regular(
                                            5,
                                        ),
                                        Regular(
                                            6,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 192,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 194,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 11,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 12,
                                },
                                FnCall {
                                    function: 8,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            9,
                                        ),
                                        Regular(
                                            10,
                                        ),
                                        Regular(
                                            13,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 194,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 17,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 18,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 15,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            16,
                                        ),
                                        Regular(
                                            19,
                                        ),
                                        Regular(
                                            20,
                                        ),
                                    ],
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        3..4,
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
                                    initial_value: 7,
                                },
                                Eval {
                                    expr_idx: 14,
                                },
                                Eval {
                                    expr_idx: 21,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 2,
                                        stmts: ArenaIdxRange(
                                            0..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 194,
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
                    path: FugitivePath(`quick_sort::partition`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::partition`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [
                                HirTemplateParameter {
                                    symbol: Type(
                                        Type {
                                            attrs: HirSymbolAttrs,
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                    },
                    body: Some(
                        62,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
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
                                                value: 192,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 3,
                                    opr: Closed(
                                        Sub,
                                    ),
                                    ropd: 4,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 193,
                                            },
                                        ),
                                    ),
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
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 8,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 9,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
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
                                Index {
                                    owner: 11,
                                    items: [
                                        14,
                                    ],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
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
                                Binary {
                                    lopd: 15,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 18,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 20,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 21,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 23,
                                    opr: AssignClosed(
                                        Sub,
                                    ),
                                    ropd: 24,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 26,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 27,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 30,
                                    opr: As,
                                    ropd: 31,
                                },
                                Index {
                                    owner: 29,
                                    items: [
                                        32,
                                    ],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
                                Index {
                                    owner: 34,
                                    items: [
                                        35,
                                    ],
                                },
                                Binary {
                                    lopd: 33,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 36,
                                },
                                Binary {
                                    lopd: 28,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 37,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    ISize(
                                        TermISizeLiteral(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 39,
                                    opr: AssignClosed(
                                        Sub,
                                    ),
                                    ropd: 40,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 42,
                                    opr: Comparison(
                                        Geq,
                                    ),
                                    ropd: 43,
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 46,
                                    opr: As,
                                    ropd: 47,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 49,
                                    opr: As,
                                    ropd: 50,
                                },
                                MethodCall {
                                    self_argument: 45,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 153,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            48,
                                        ),
                                        Regular(
                                            51,
                                        ),
                                    ],
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 190,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 54,
                                    opr: As,
                                    ropd: 55,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                Binary {
                                    lopd: 57,
                                    opr: As,
                                    ropd: 58,
                                },
                                MethodCall {
                                    self_argument: 53,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 153,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            56,
                                        ),
                                        Regular(
                                            59,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        9..15,
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
                                    expr_idx: 41,
                                },
                                Break,
                                Eval {
                                    expr_idx: 52,
                                },
                                Eval {
                                    expr_idx: 10,
                                },
                                While {
                                    condition: 19,
                                    stmts: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                                Eval {
                                    expr_idx: 25,
                                },
                                While {
                                    condition: 38,
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 44,
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
                                    initial_value: 5,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 6,
                                },
                                While {
                                    condition: 7,
                                    stmts: ArenaIdxRange(
                                        4..9,
                                    ),
                                },
                                Eval {
                                    expr_idx: 60,
                                },
                                Eval {
                                    expr_idx: 61,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 196,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 197,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 198,
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
                    path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        12,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                Literal(
                                    I32(
                                        4,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        65,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        31,
                                    ),
                                ),
                                Prefix {
                                    opr: Minus,
                                    opd: 3,
                                },
                                Literal(
                                    I32(
                                        0,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        99,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        2,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        83,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        782,
                                    ),
                                ),
                                Literal(
                                    I32(
                                        1,
                                    ),
                                ),
                                List {
                                    items: [
                                        0,
                                        1,
                                        2,
                                        4,
                                        5,
                                        6,
                                        7,
                                        8,
                                        9,
                                        10,
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
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    initial_value: 11,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 200,
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
                    path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
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
                                Literal(
                                    String(
                                        StringLiteral(
                                            Id {
                                                value: 18,
                                            },
                                        ),
                                    ),
                                ),
                                Literal(
                                    String(
                                        StringLiteral(
                                            Id {
                                                value: 19,
                                            },
                                        ),
                                    ),
                                ),
                                Literal(
                                    String(
                                        StringLiteral(
                                            Id {
                                                value: 20,
                                            },
                                        ),
                                    ),
                                ),
                                Literal(
                                    String(
                                        StringLiteral(
                                            Id {
                                                value: 21,
                                            },
                                        ),
                                    ),
                                ),
                                Literal(
                                    String(
                                        StringLiteral(
                                            Id {
                                                value: 22,
                                            },
                                        ),
                                    ),
                                ),
                                Literal(
                                    String(
                                        StringLiteral(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                ),
                                List {
                                    items: [
                                        0,
                                        1,
                                        2,
                                        3,
                                        4,
                                        5,
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
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: None,
                                    },
                                    initial_value: 6,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 202,
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