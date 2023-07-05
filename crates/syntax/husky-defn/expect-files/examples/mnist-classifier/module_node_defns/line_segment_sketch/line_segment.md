Ok(
    [
        NodeDefn::ModuleItem(
            ModuleItemNodeDefn::Type(
                TypeNodeDefn::PropsStruct(
                    PropsStructTypeNodeDefn {
                        node_path: TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                        node_decl: PropsStructTypeNodeDecl {
                            node_path: TypeNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 16,
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            lcurl: Ok(
                                PropsStructLeftCurlyBrace(
                                    LeftCurlyBraceToken(
                                        TokenIdx(
                                            9,
                                        ),
                                    ),
                                ),
                            ),
                            fields: Ok(
                                SeparatedSmallList {
                                    elements: [
                                        PropsFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
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
                                            colon: ColonToken(
                                                TokenIdx(
                                                    15,
                                                ),
                                            ),
                                            ty_expr_idx: 1,
                                            initialization: None,
                                        },
                                    ],
                                    separators: [
                                        CommaToken(
                                            TokenIdx(
                                                13,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                17,
                                            ),
                                        ),
                                    ],
                                    phantom: PhantomData<husky_decl::error::NodeDeclError>,
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRightCurlyBraceToken(
                                    RightCurlyBraceToken(
                                        TokenIdx(
                                            18,
                                        ),
                                    ),
                                ),
                            ),
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
        NodeDefn::AssociatedItem(
            AssociatedItemNodeDefn::TypeItem(
                TypeItemNodeDefn::MethodFn(
                    TypeMethodFnNodeDefn {
                        node_path: TypeItemNodePath {
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
                        node_decl: TypeMethodFnNodeDecl {
                            node_path: TypeItemNodePath {
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
                            node: TypeItemNode {
                                node_path: TypeItemNodePath {
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
                                ast_idx: 13,
                                ident: `displacement`,
                                item_kind: MethodFn,
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch::line_segment`,
                                ),
                                is_generic: false,
                            },
                            ast_idx: 13,
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            explicit_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            24,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    explicit_parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            25,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            26,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExpr {
                                        expr: 0,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            28,
                                        ),
                                    },
                                ),
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
        NodeDefn::AssociatedItem(
            AssociatedItemNodeDefn::TypeItem(
                TypeItemNodeDefn::MethodFn(
                    TypeMethodFnNodeDefn {
                        node_path: TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `dist_to_point`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        node_decl: TypeMethodFnNodeDecl {
                            node_path: TypeItemNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypeItemPath {
                                        impl_block: TypeImplBlockPath {
                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            disambiguator: 0,
                                        },
                                        ident: `dist_to_point`,
                                        item_kind: MethodFn,
                                    },
                                    disambiguator: 0,
                                },
                            },
                            node: TypeItemNode {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `dist_to_point`,
                                            item_kind: MethodFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 14,
                                ident: `dist_to_point`,
                                item_kind: MethodFn,
                                visibility: Scope::PubUnder(
                                    `mnist_classifier::line_segment_sketch::line_segment`,
                                ),
                                is_generic: false,
                            },
                            ast_idx: 14,
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            explicit_parameter_decl_list: Err(
                                NodeDeclError::Original(
                                    OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                        TokenStreamState {
                                            next_token_idx: TokenIdx(
                                                42,
                                            ),
                                            drained: false,
                                        },
                                    ),
                                ),
                            ),
                            curry_token: Ok(
                                None,
                            ),
                            return_ty: Ok(
                                None,
                            ),
                            eol_colon: Err(
                                NodeDeclError::Original(
                                    OriginalNodeDeclError::ExpectedEolColon(
                                        TokenStreamState {
                                            next_token_idx: TokenIdx(
                                                42,
                                            ),
                                            drained: false,
                                        },
                                    ),
                                ),
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
                                                            ident: `dist_to_point`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [],
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
                                    roots: [],
                                },
                            },
                        },
                        body: Some(
                            31,
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
                                                                    ident: `dist_to_point`,
                                                                    item_kind: MethodFn,
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [],
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
                                            roots: [],
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
                                                        ident: `dist_to_point`,
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
                                                52,
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                53,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    54,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                55,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                56,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                60,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                61,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    62,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 421,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 3,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `to`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                65,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                67,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 422,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 423,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 6,
                                            dot_token_idx: TokenIdx(
                                                70,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `dot`,
                                                token_idx: TokenIdx(
                                                    71,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                72,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 7,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                74,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                76,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 8,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                75,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 423,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 11,
                                            dot_token_idx: TokenIdx(
                                                79,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    80,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                81,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                82,
                                            ),
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                88,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 13,
                                            dot_token_idx: TokenIdx(
                                                89,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    90,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 421,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 14,
                                            dot_token_idx: TokenIdx(
                                                91,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `to`,
                                                token_idx: TokenIdx(
                                                    92,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                93,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 15,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                95,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        97,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 422,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 424,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                98,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `dot`,
                                                token_idx: TokenIdx(
                                                    99,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                100,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 18,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                102,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                104,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 19,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                103,
                                            ),
                                            ropd: 20,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 424,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 22,
                                            dot_token_idx: TokenIdx(
                                                107,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    108,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                109,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                110,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 422,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        117,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 423,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 24,
                                            dot_token_idx: TokenIdx(
                                                114,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `cross`,
                                                token_idx: TokenIdx(
                                                    115,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                116,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 25,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                118,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        124,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 422,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 26,
                                            dot_token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
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
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 27,
                                            dot_token_idx: TokenIdx(
                                                125,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                127,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                128,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 28,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                123,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                5..8,
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
                                            expr_idx: 12,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 23,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 30,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    85,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                86,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        87,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 16,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        96,
                                                    ),
                                                },
                                                condition: Ok(
                                                    21,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                105,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            111,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    112,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    49,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                50,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        51,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    57,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                58,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        59,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 5,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                },
                                                condition: Ok(
                                                    10,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                77,
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
                                            elif_branches: [],
                                            else_branch: Some(
                                                ElseBranch {
                                                    else_token: ElseToken {
                                                        token_idx: TokenIdx(
                                                            83,
                                                        ),
                                                    },
                                                    eol_colon: Ok(
                                                        Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    84,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            3..5,
                                                        ),
                                                    ),
                                                },
                                            ),
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 12,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 16,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 23,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 30,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 31,
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