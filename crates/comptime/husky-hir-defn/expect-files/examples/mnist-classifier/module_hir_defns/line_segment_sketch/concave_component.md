[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `line_segment_sketch`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 53,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `strokes`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 56,
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
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        61,
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
                                                value: 217,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 2,
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
                                Literal(
                                    I32(
                                        1,
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
                                                value: 23,
                                            },
                                        ),
                                    ),
                                },
                                Prefix {
                                    opr: Minus,
                                    opd: 7,
                                },
                                Binary {
                                    lopd: 6,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 8,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 217,
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
                                    ],
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 13,
                                },
                                Binary {
                                    lopd: 9,
                                    opr: ShortCircuitLogic(
                                        And,
                                    ),
                                    ropd: 14,
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
                                    opd: 16,
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
                                                value: 419,
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
                                    lopd: 20,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 21,
                                },
                                Binary {
                                    lopd: 19,
                                    opr: Comparison(
                                        Less,
                                    ),
                                    ropd: 22,
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
                                    lopd: 25,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 26,
                                },
                                Binary {
                                    lopd: 24,
                                    opr: Comparison(
                                        Leq,
                                    ),
                                    ropd: 27,
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Fugitive(
                                            FugitivePath(
                                                Id {
                                                    value: 24,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 217,
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
                                    function: 29,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            30,
                                        ),
                                        Regular(
                                            31,
                                        ),
                                    ],
                                },
                                Prefix {
                                    opr: Not,
                                    opd: 32,
                                },
                                Binary {
                                    lopd: 28,
                                    opr: ShortCircuitLogic(
                                        And,
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
                                Suffix {
                                    opd: 35,
                                    opr: Incr,
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
                                    lopd: 38,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 39,
                                },
                                Binary {
                                    lopd: 37,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 40,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 375,
                                            },
                                        ),
                                    ),
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 57,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 217,
                                            },
                                        ),
                                    ),
                                },
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 217,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 45,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
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
                                MethodCall {
                                    self_argument: 46,
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
                                            47,
                                        ),
                                        Regular(
                                            48,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 43,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            44,
                                        ),
                                        Regular(
                                            49,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 42,
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
                                            50,
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
                                    lopd: 52,
                                    opr: Assign,
                                    ropd: 53,
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
                                    lopd: 56,
                                    opr: Closed(
                                        Add,
                                    ),
                                    ropd: 57,
                                },
                                Binary {
                                    lopd: 55,
                                    opr: Assign,
                                    ropd: 58,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 375,
                                            },
                                        ),
                                    ),
                                },
                                Block {
                                    stmts: ArenaIdxRange(
                                        7..15,
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 17,
                                },
                                Eval {
                                    expr_idx: 36,
                                },
                                Eval {
                                    expr_idx: 51,
                                },
                                While {
                                    condition: 34,
                                    stmts: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 41,
                                        stmts: ArenaIdxRange(
                                            2..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                                Eval {
                                    expr_idx: 54,
                                },
                                Eval {
                                    expr_idx: 59,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 57,
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
                                    initial_value: 3,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 4,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 3,
                                        ty: None,
                                    },
                                    initial_value: 5,
                                },
                                While {
                                    condition: 15,
                                    stmts: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 18,
                                },
                                While {
                                    condition: 23,
                                    stmts: ArenaIdxRange(
                                        3..7,
                                    ),
                                },
                                Return {
                                    result: 60,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 375,
                                            },
                                        ),
                                    ),
                                },
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
                                                value: 419,
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
                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                            value: 39,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                        value: 39,
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
                                                value: 374,
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
                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 39,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `norm`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
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
                        2,
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
                                                value: 410,
                                            },
                                        ),
                                    ),
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
                                    expr_idx: 1,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `rel_norm`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `rel_norm`,
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
                        6,
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
                                                value: 352,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                MethodCall {
                                    self_argument: 2,
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
                                MethodCall {
                                    self_argument: 3,
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
                                    lopd: 1,
                                    opr: Closed(
                                        Div,
                                    ),
                                    ropd: 4,
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
                                    expr_idx: 5,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `hausdorff_norm`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `hausdorff_norm`,
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
                        32,
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
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 2,
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
                                    opd: 3,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 4,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                SelfValue,
                                MethodCall {
                                    self_argument: 6,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 221,
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
                                                value: 413,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 8,
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
                                MethodCall {
                                    self_argument: 9,
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
                                    owner: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 12,
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
                                SelfValue,
                                Field {
                                    owner: 14,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 15,
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
                                SelfValue,
                                Field {
                                    owner: 17,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
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
                                    owner: 18,
                                    items: [
                                        19,
                                    ],
                                },
                                Field {
                                    owner: 20,
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
                                                value: 413,
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
                                MethodCall {
                                    self_argument: 22,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 415,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            23,
                                        ),
                                    ],
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 414,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 410,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 25,
                                    opr: Comparison(
                                        Greater,
                                    ),
                                    ropd: 26,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 410,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 414,
                                            },
                                        ),
                                    ),
                                },
                                Binary {
                                    lopd: 28,
                                    opr: Assign,
                                    ropd: 29,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 410,
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
                        stmt_arena: Arena {
                            data: [
                                Eval {
                                    expr_idx: 30,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 4,
                                        ty: None,
                                    },
                                    initial_value: 21,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 5,
                                        ty: None,
                                    },
                                    initial_value: 24,
                                },
                                IfElse {
                                    if_branch: HirEagerIfBranch {
                                        condition: 27,
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
                                    initial_value: 10,
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
                                                    13,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    16,
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
                                Return {
                                    result: 31,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 410,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 412,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 413,
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
                                                value: 297,
                                            },
                                        ),
                                    ),
                                },
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 414,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `angle_change`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `angle_change`,
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
                        29,
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
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
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
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 4,
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
                                Index {
                                    owner: 2,
                                    items: [
                                        5,
                                    ],
                                },
                                MethodCall {
                                    self_argument: 6,
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
                                    owner: 8,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 9,
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
                                SelfValue,
                                Field {
                                    owner: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 12,
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
                                SelfValue,
                                Field {
                                    owner: 14,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
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
                                                value: 301,
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
                                                value: 349,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 387,
                                            },
                                        ),
                                    ),
                                },
                                Literal(
                                    Bool(
                                        true,
                                    ),
                                ),
                                MethodCall {
                                    self_argument: 20,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 361,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            21,
                                        ),
                                        Regular(
                                            22,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 19,
                                    opr: AssignClosed(
                                        Add,
                                    ),
                                    ropd: 23,
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
                                    lopd: 25,
                                    opr: Assign,
                                    ropd: 26,
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
                                Block {
                                    stmts: ArenaIdxRange(
                                        3..7,
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
                                    initial_value: 18,
                                },
                                Eval {
                                    expr_idx: 24,
                                },
                                Eval {
                                    expr_idx: 27,
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
                                    initial_value: 7,
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
                                                    10,
                                                ),
                                                kind: LowerOpen,
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
                                        0..3,
                                    ),
                                },
                                Return {
                                    result: 28,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 349,
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
                                                value: 387,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        58,
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
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
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
                                    opd: 2,
                                    opr: UnwrapOrComposeWithNot,
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
                                                value: 292,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 9,
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
                                                value: 292,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 11,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
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
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 14,
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
                                SelfValue,
                                Field {
                                    owner: 16,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 17,
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
                                SelfValue,
                                Field {
                                    owner: 19,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
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
                                    owner: 20,
                                    items: [
                                        21,
                                    ],
                                },
                                Field {
                                    owner: 22,
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
                                                value: 297,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 26,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 25,
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
                                            27,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 24,
                                    opr: Assign,
                                    ropd: 28,
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
                                                value: 297,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 32,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 274,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 31,
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
                                            33,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 30,
                                    opr: Assign,
                                    ropd: 34,
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
                                                value: 297,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 38,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 37,
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
                                            39,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 36,
                                    opr: Assign,
                                    ropd: 40,
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
                                                value: 297,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 44,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 275,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 43,
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
                                            45,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 42,
                                    opr: Assign,
                                    ropd: 46,
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
                                                value: 293,
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
                                FnCall {
                                    function: 49,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            50,
                                        ),
                                        Regular(
                                            51,
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
                                FnCall {
                                    function: 53,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            54,
                                        ),
                                        Regular(
                                            55,
                                        ),
                                    ],
                                },
                                FnCall {
                                    function: 48,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            52,
                                        ),
                                        Regular(
                                            56,
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
                                    initial_value: 23,
                                },
                                Eval {
                                    expr_idx: 29,
                                },
                                Eval {
                                    expr_idx: 35,
                                },
                                Eval {
                                    expr_idx: 41,
                                },
                                Eval {
                                    expr_idx: 47,
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
                                                    value: 259,
                                                },
                                            ),
                                        ),
                                        range: HirEagerForBetweenRange {
                                            initial_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    15,
                                                ),
                                                kind: LowerClosed,
                                            },
                                            final_boundary: HirEagerForBetweenLoopBoundary {
                                                bound_expr: Some(
                                                    18,
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
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 297,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `relative_bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        6,
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
                                                value: 217,
                                            },
                                        ),
                                    ),
                                },
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 290,
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
                                                value: 290,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 2,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 299,
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
                                        0..1,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `line_segment`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `line_segment`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
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
                                        value: 39,
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
                                    value: 58,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        14,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 59,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                SelfValue,
                                Field {
                                    owner: 1,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 2,
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
                                    opd: 3,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 4,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 146,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 5,
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
                                SelfValue,
                                Field {
                                    owner: 7,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 8,
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
                                    opd: 9,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                Field {
                                    owner: 10,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 147,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 11,
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
                                FnCall {
                                    function: 0,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            6,
                                        ),
                                        Regular(
                                            12,
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
                                    expr_idx: 13,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `start`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `start`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
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
                                        value: 39,
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
                                    value: 46,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        6,
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
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
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
                                    opd: 2,
                                    opr: UnwrapOrComposeWithNot,
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
                                MethodCall {
                                    self_argument: 4,
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
                                    expr_idx: 5,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `end`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `end`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
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
                                        value: 39,
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
                                    value: 46,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        6,
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
                                                value: 374,
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
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 2,
                                    opr: UnwrapOrComposeWithNot,
                                },
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
                                    self_argument: 4,
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
                                    expr_idx: 5,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
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
                                        value: 39,
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
                        3,
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
                                                value: 221,
                                            },
                                        ),
                                    ),
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                MethodCall {
                                    self_argument: 1,
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
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `start_tangent`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `start_tangent`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
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
                                        value: 39,
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
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 1,
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
                                    opd: 2,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                MethodCall {
                                    self_argument: 3,
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
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `end_tangent`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `end_tangent`,
                            item_kind: MethodFn,
                        },
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 39,
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
                                        value: 39,
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
                                                value: 374,
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
                                    generic_arguments: None,
                                    item_groups: [],
                                },
                                Suffix {
                                    opd: 2,
                                    opr: UnwrapOrComposeWithNot,
                                },
                                MethodCall {
                                    self_argument: 3,
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
]