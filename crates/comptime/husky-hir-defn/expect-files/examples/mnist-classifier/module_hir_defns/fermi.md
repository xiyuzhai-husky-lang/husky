[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                        fields: [
                            PropsFieldHirDecl {
                                ident: `matches`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `others`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 43,
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
                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        17,
                    ),
                    hir_expr_region: HirEagerExprRegion {
                        expr_arena: Arena {
                            data: [
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 374,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 0,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 436,
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
                                InheritedSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 436,
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
                                    owner: 5,
                                    items: [
                                        6,
                                    ],
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
                                                value: 433,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 437,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 9,
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
                                            10,
                                        ),
                                    ],
                                },
                                MethodCall {
                                    self_argument: 8,
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
                                            11,
                                        ),
                                    ],
                                },
                                PrincipalEntityPath(
                                    MajorItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 60,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
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
                                                value: 433,
                                            },
                                        ),
                                    ),
                                },
                                FnCall {
                                    function: 13,
                                    generic_arguments: None,
                                    item_groups: [
                                        Regular(
                                            14,
                                        ),
                                        Regular(
                                            15,
                                        ),
                                    ],
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
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 2,
                                        ty: None,
                                    },
                                    initial_value: 7,
                                },
                                Eval {
                                    expr_idx: 12,
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
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 42,
                                                    },
                                                ),
                                            ),
                                        ),
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
                                        0..2,
                                    ),
                                },
                                Return {
                                    result: 16,
                                },
                            ],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 433,
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
                                                value: 437,
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
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath {
                    module_path: `mnist_classifier::fermi`,
                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 44,
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
                            module_path: `mnist_classifier::fermi`,
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `norm`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                        14,
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
                                                value: 433,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
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
                                                value: 433,
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
                                    owner: 7,
                                    items: [
                                        8,
                                    ],
                                },
                                Field {
                                    owner: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 5,
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
                                                value: 351,
                                            },
                                        ),
                                    ),
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
                                    expr_idx: 12,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 14,
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
                                        0..1,
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
                                                value: 351,
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
                            module_path: `mnist_classifier::fermi`,
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `rel_norm`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                        14,
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
                                                value: 433,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
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
                                                value: 433,
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
                                    owner: 7,
                                    items: [
                                        8,
                                    ],
                                },
                                Field {
                                    owner: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 410,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 5,
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
                                                value: 351,
                                            },
                                        ),
                                    ),
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
                                    expr_idx: 12,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 14,
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
                                        0..1,
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
                                                value: 351,
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
                            module_path: `mnist_classifier::fermi`,
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `angle_change_norm`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `angle_change_norm`,
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
                                                value: 433,
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
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
                                            },
                                        ),
                                    ),
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
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
                                                value: 433,
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
                                    owner: 7,
                                    items: [
                                        8,
                                    ],
                                },
                                Field {
                                    owner: 9,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 348,
                                            },
                                        ),
                                    ),
                                },
                                MethodCall {
                                    self_argument: 10,
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
                                MethodCall {
                                    self_argument: 5,
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
                                            11,
                                        ),
                                    ],
                                },
                                Binary {
                                    lopd: 4,
                                    opr: Assign,
                                    ropd: 12,
                                },
                                CurrentSymbol {
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 351,
                                            },
                                        ),
                                    ),
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
                                    expr_idx: 13,
                                },
                                Let {
                                    pattern: HirEagerLetVariablesPattern {
                                        pattern_expr_idx: 0,
                                        ty: Some(
                                            PathLeading(
                                                HirTypePathLeading(
                                                    Id {
                                                        value: 14,
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
                                                value: 351,
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