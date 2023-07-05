Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `matches`,
                                        token_idx: TokenIdx(
                                            8,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            9,
                                        ),
                                    ),
                                    ty_expr_idx: 4,
                                    initialization: None,
                                },
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `others`,
                                        token_idx: TokenIdx(
                                            16,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            17,
                                        ),
                                    ),
                                    ty_expr_idx: 8,
                                    initialization: None,
                                },
                            ],
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ModuleItem(
                                            ModuleItemNodePath::Type(
                                                TypeNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    13,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::List {
                                                lbox_token_idx: TokenIdx(
                                                    10,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    11,
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    12,
                                                ),
                                                opd: 1,
                                            },
                                            Expr::ExplicitApplication {
                                                function: 2,
                                                argument: 3,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::List {
                                                lbox_token_idx: TokenIdx(
                                                    18,
                                                ),
                                                items: [],
                                                rbox_token_idx: TokenIdx(
                                                    19,
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    20,
                                                ),
                                                opd: 5,
                                            },
                                            Expr::ExplicitApplication {
                                                function: 6,
                                                argument: 7,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            21,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 220,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        8,
                                                    ),
                                                },
                                            },
                                            expr_idx: 4,
                                        },
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 426,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                            },
                                            expr_idx: 8,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ImplBlock(
            ImplBlockDecl::Type(
                TypeImplBlockDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::fermi`,
                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        disambiguator: 0,
                    },
                    implicit_parameters: [],
                    ty_expr: TypeExpr {
                        expr: 0,
                    },
                    expr_region: ExprRegion {
                        data: ExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntityNodePath::ImplBlock(
                                    ImplBlockNodePath::TypeImplBlock(
                                        TypeImplBlockNodePath {
                                            path: TypeImplBlockPath {
                                                module_path: `mnist_classifier::fermi`,
                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    Expr::PrincipalEntityPath {
                                        entity_path_expr: 0,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            principal_entity_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `FermiMatchResult`,
                                                token_idx: TokenIdx(
                                                    25,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: PatternExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_infos: [],
                                pattern_symbol_arena: Arena {
                                    data: [],
                                },
                                pattern_symbol_maps: [],
                                pattern_symbol_modifiers: ArenaMap {
                                    data: [],
                                },
                            },
                            symbol_region: SymbolRegion {
                                inherited_symbol_arena: Arena {
                                    data: [],
                                },
                                current_symbol_arena: Arena {
                                    data: [],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                ExprRoot {
                                    kind: SelfType,
                                    expr_idx: 0,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `FermiMatchResult`,
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: PatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `norm`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            16,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `FermiMatchResult`,
                                                                            token_idx: TokenIdx(
                                                                                25,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        stmt_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_region: PatternExprRegion {
                                                            pattern_expr_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
                                                            inherited_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            current_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::fermi`,
                                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `norm`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    30,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::fermi`,
                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `norm`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                36,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                40,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                41,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    42,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                38,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                43,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    44,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                45,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                46,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                39,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                50,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                54,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                55,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    56,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                58,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 8,
                                            lbox_token_idx: TokenIdx(
                                                57,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                59,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                60,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                48,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                51,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    52,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                53,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 11,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                62,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 12,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                49,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                64,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 14,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    32,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        35,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 0,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    37,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    38,
                                                ),
                                                frame_var_expr_idx: 3,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
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
                                            frame_var_symbol_idx: 1,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                            },
                                            result: 15,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            33,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `norm`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `norm`,
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    35,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            65,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `norm`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    48,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            63,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 3,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 0,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 14,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 15,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 16,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `rel_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `rel_norm`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `FermiMatchResult`,
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: PatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `rel_norm`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            68,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            16,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `FermiMatchResult`,
                                                                            token_idx: TokenIdx(
                                                                                25,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        stmt_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_region: PatternExprRegion {
                                                            pattern_expr_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
                                                            inherited_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            current_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::fermi`,
                                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `rel_norm`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    68,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::fermi`,
                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `rel_norm`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                74,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                78,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                79,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    80,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                76,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                81,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    82,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                83,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                84,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                77,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                92,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                93,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    94,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                96,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 8,
                                            lbox_token_idx: TokenIdx(
                                                95,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                97,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                98,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    99,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                86,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                89,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    90,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                91,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 11,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                100,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 12,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                87,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                102,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 14,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        73,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 0,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    75,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    76,
                                                ),
                                                frame_var_expr_idx: 3,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
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
                                            frame_var_symbol_idx: 1,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            85,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    101,
                                                ),
                                            },
                                            result: 15,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            71,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `norm`,
                                                    token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `norm`,
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    73,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            103,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `norm`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    86,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            101,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 3,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 0,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 14,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 15,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 16,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MemoizedField(
                    TypeMemoizedFieldDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `angle_change_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `angle_change_norm`,
                                item_kind: MemoizedField,
                            },
                            memo_ty: Some(
                                FormTypeExpr {
                                    expr: 0,
                                },
                            ),
                            expr: None,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: Some(
                                        ExprRegion {
                                            data: ExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntityNodePath::ImplBlock(
                                                        ImplBlockNodePath::TypeImplBlock(
                                                            TypeImplBlockNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::PrincipalEntityPath {
                                                            entity_path_expr: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                principal_entity_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `FermiMatchResult`,
                                                                    token_idx: TokenIdx(
                                                                        25,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: PatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: SymbolRegion {
                                                    inherited_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntityNodePath::AssociatedItem(
                                            AssociatedItemNodePath::TypeItem(
                                                TypeItemNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `angle_change_norm`,
                                                            item_kind: MemoizedField,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: VarType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            17,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            EntityNodePath::ImplBlock(
                                                                ImplBlockNodePath::TypeImplBlock(
                                                                    TypeImplBlockNodePath {
                                                                        path: TypeImplBlockPath {
                                                                            module_path: `mnist_classifier::fermi`,
                                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::PrincipalEntityPath {
                                                                    entity_path_expr: 0,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        principal_entity_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `FermiMatchResult`,
                                                                            token_idx: TokenIdx(
                                                                                25,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        stmt_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_region: PatternExprRegion {
                                                            pattern_expr_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
                                                            pattern_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_modifiers: ArenaMap {
                                                                data: [],
                                                            },
                                                        },
                                                        symbol_region: SymbolRegion {
                                                            inherited_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            current_symbol_arena: Arena {
                                                                data: [],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [],
                                                        },
                                                        roots: [
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntityNodePath::AssociatedItem(
                                                    AssociatedItemNodePath::TypeItem(
                                                        TypeItemNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TypeItemPath {
                                                                    impl_block: TypeImplBlockPath {
                                                                        module_path: `mnist_classifier::fermi`,
                                                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `angle_change_norm`,
                                                                    item_kind: MemoizedField,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    106,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
                                                pattern_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_symbol_maps: [],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [],
                                                },
                                            },
                                            symbol_region: SymbolRegion {
                                                inherited_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: VarType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::AssociatedItem(
                                        AssociatedItemNodePath::TypeItem(
                                            TypeItemNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `mnist_classifier::fermi`,
                                                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `angle_change_norm`,
                                                        item_kind: MemoizedField,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                112,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                116,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                117,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                114,
                                            ),
                                            ident: `i`,
                                            frame_var_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 2,
                                            dot_token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                121,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                122,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                115,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                126,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                130,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                131,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                134,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::IndexOrCompositionWithList {
                                            owner: 8,
                                            lbox_token_idx: TokenIdx(
                                                133,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 9,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                135,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                136,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 11,
                                            dot_token_idx: TokenIdx(
                                                138,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                140,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                141,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                124,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                127,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                129,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 12,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                142,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 13,
                                            opr: Assign,
                                            opr_token_idx: TokenIdx(
                                                125,
                                            ),
                                            ropd: 14,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `norm`,
                                            token_idx: TokenIdx(
                                                144,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 15,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    108,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        111,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 0,
                                        },
                                        Stmt::ForBetween {
                                            for_token: StmtForToken {
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    114,
                                                ),
                                                frame_var_expr_idx: 3,
                                                frame_var_ident: `i`,
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
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
                                            frame_var_symbol_idx: 1,
                                            eol_colon: Ok(
                                                EolToken::Colon(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    143,
                                                ),
                                            },
                                            result: 16,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: Mut(
                                                    MutToken {
                                                        token_idx: TokenIdx(
                                                            109,
                                                        ),
                                                    },
                                                ),
                                                ident_token: IdentToken {
                                                    ident: `norm`,
                                                    token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Move,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `norm`,
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Mut,
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Mut,
                                                access_start: TokenIdx(
                                                    111,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            145,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `norm`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    124,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            143,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable {
                                                    ident: `i`,
                                                    expr_idx: 3,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: True,
                                    pattern_ty_constraints: [
                                        (
                                            FrameVariable,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 0,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 15,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 16,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 17,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)