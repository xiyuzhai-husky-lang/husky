Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `start`,
                                        token_idx: TokenIdx(
                                            10,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            11,
                                        ),
                                    ),
                                    ty_expr_idx: 0,
                                    initialization: None,
                                },
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `end`,
                                        token_idx: TokenIdx(
                                            14,
                                        ),
                                    },
                                    colon: ColonToken(
                                        TokenIdx(
                                            15,
                                        ),
                                    ),
                                    ty_expr_idx: 1,
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
                                                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ident: `Point2d`,
                                                        token_idx: TokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Point2d`,
                                                        token_idx: TokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                value: 284,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        10,
                                                    ),
                                                },
                                            },
                                            expr_idx: 0,
                                        },
                                        ExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 285,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            },
                                            expr_idx: 1,
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
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    20,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `displacement`,
                                item_kind: MethodFn,
                            },
                            implicit_parameters: [],
                            self_parameter: None,
                            explicit_parameters: [],
                            return_ty: Some(
                                ReturnTypeExpr {
                                    expr: 0,
                                },
                            ),
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
                                                                    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                    ident: `LineSegment`,
                                                                    token_idx: TokenIdx(
                                                                        20,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `displacement`,
                                                            item_kind: MethodFn,
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
                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                        ident: `Vector2d`,
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                            kind: ReturnType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            5,
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
                                                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                            ident: `LineSegment`,
                                                                            token_idx: TokenIdx(
                                                                                20,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `displacement`,
                                                                    item_kind: MethodFn,
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
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                ident: `Vector2d`,
                                                                token_idx: TokenIdx(
                                                                    27,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                    kind: ReturnType,
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
                                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `displacement`,
                                                        item_kind: MethodFn,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::SelfValue(
                                            TokenIdx(
                                                29,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                30,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                35,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                36,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    37,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                32,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `to`,
                                                token_idx: TokenIdx(
                                                    33,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                34,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                38,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
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
                                            expr_idx: 4,
                                        },
                                    ],
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
                                        kind: EvalExpr,
                                        expr_idx: 4,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 5,
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