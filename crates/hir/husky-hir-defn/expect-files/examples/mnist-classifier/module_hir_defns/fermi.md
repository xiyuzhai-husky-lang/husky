[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsFieldHirDecl {
                                ident: `matches`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        template_arguments: [
                                            Type(
                                                PathLeading(
                                                    HirTypePathLeading(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                    },
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `others`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        template_arguments: [
                                            Type(
                                                PathLeading(
                                                    HirTypePathLeading(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                    },
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
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 48,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 49,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 375,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            18,
                            HirEagerExprRegion {
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        InheritedSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 375,
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
                                        InheritedSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 437,
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
                                        InheritedSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 437,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 248,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 434,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 438,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 248,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 434,
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
                                hir_eager_stmt_arena: Arena {
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
                                            discarded: false,
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
                                                                value: 45,
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
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 434,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 248,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 438,
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
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::fermi`,
                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        disambiguator: 0,
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                },
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
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
                            15,
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
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 434,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                                        value: 434,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
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
                                                        value: 352,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 13,
                                            discarded: false,
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
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
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
                            15,
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
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 434,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                                        value: 434,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
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
                                                        value: 411,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 13,
                                            discarded: false,
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
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
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
                            16,
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
                                        SelfType,
                                        Field {
                                            owner_hir_expr_idx: 2,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 434,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                                        value: 434,
                                                    },
                                                ),
                                            ),
                                        },
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 260,
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
                                                        value: 349,
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
                                        CurrentSynSymbol {
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 14,
                                            discarded: false,
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
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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