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
                                            value: 44,
                                        },
                                    ),
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `others`,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 45,
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
            FugitiveHirDefn::FunctionFn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            18,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 376,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 1,
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
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 438,
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
                                        InheritedSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 438,
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
                                                        value: 435,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 439,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 10,
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
                                                    11,
                                                ),
                                            ],
                                        },
                                        MethodCall {
                                            self_argument: 9,
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
                                                    12,
                                                ),
                                            ],
                                        },
                                        PrincipalEntityPath(
                                            MajorItem(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 62,
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
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 435,
                                                    },
                                                ),
                                            ),
                                        },
                                        FnCall {
                                            function_hir_expr_idx: 14,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    15,
                                                ),
                                                Regular(
                                                    16,
                                                ),
                                            ],
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
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Eval {
                                            expr_idx: 13,
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
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 44,
                                                            },
                                                        ),
                                                    ),
                                                ),
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
                                                        bound_expr: None,
                                                        kind: LowerClosed,
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
                                            result: 17,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 435,
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
                                        Ident {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 439,
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
                            value: 46,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            15,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 435,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 3,
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
                                                        value: 353,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
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
                                                        value: 435,
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
                                            owner_hir_expr_idx: 8,
                                            items: [
                                                9,
                                            ],
                                        },
                                        Field {
                                            owner_hir_expr_idx: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 6,
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
                                                    11,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 5,
                                            opr: Assign,
                                            ropd: 12,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
                                                    },
                                                ),
                                            ),
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
                                            expr_idx: 13,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
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
                                                1..2,
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
                                                        value: 353,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            15,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 435,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 3,
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
                                                        value: 353,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
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
                                                        value: 435,
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
                                            owner_hir_expr_idx: 8,
                                            items: [
                                                9,
                                            ],
                                        },
                                        Field {
                                            owner_hir_expr_idx: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 412,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 6,
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
                                                    11,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 5,
                                            opr: Assign,
                                            ropd: 12,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
                                                    },
                                                ),
                                            ),
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
                                            expr_idx: 13,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
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
                                                1..2,
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
                                                        value: 353,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            16,
                            HirEagerExprRegion {
                                expr_arena: Arena {
                                    data: [
                                        Literal(
                                            F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 435,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 3,
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
                                                        value: 353,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
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
                                                        value: 435,
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
                                            owner_hir_expr_idx: 8,
                                            items: [
                                                9,
                                            ],
                                        },
                                        Field {
                                            owner_hir_expr_idx: 10,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 350,
                                                    },
                                                ),
                                            ),
                                        },
                                        MethodCall {
                                            self_argument: 11,
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
                                        MethodCall {
                                            self_argument: 6,
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
                                                    12,
                                                ),
                                            ],
                                        },
                                        Binary {
                                            lopd: 5,
                                            opr: Assign,
                                            ropd: 13,
                                        },
                                        CurrentSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 353,
                                                    },
                                                ),
                                            ),
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
                                            expr_idx: 14,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
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
                                                        value: 353,
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