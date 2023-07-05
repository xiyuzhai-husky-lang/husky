Ok(
    NodeDeclSheet {
        [salsa id]: 39,
        decls: [
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Type(
                        TypeNodeDecl::PropsStruct(
                            PropsStructTypeNodeDecl {
                                node_path: TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 78,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: Ok(
                                    PropsStructLeftCurlyBrace(
                                        LeftCurlyBraceToken(
                                            TokenIdx(
                                                3,
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
                                                                value: 254,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        4,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        5,
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
                                                                value: 255,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        8,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                        ],
                                        separators: [
                                            CommaToken(
                                                TokenIdx(
                                                    7,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    11,
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
                                                12,
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
                                                            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
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
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                10,
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 254,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            4,
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
                                                                    value: 255,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Type(
                        TypeNodeDecl::PropsStruct(
                            PropsStructTypeNodeDecl {
                                node_path: TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 80,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: Ok(
                                    PropsStructLeftCurlyBrace(
                                        LeftCurlyBraceToken(
                                            TokenIdx(
                                                145,
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
                                                                value: 254,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        146,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        147,
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
                                                                value: 255,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        151,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                        ],
                                        separators: [
                                            CommaToken(
                                                TokenIdx(
                                                    149,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    153,
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
                                                154,
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
                                                            path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
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
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
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
                                                                148,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                152,
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 254,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            146,
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
                                                                    value: 255,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            150,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Type(
                        TypeNodeDecl::PropsStruct(
                            PropsStructTypeNodeDecl {
                                node_path: TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 81,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: Ok(
                                    PropsStructLeftCurlyBrace(
                                        LeftCurlyBraceToken(
                                            TokenIdx(
                                                158,
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
                                                                value: 254,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        160,
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
                                                                value: 255,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        164,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                        ],
                                        separators: [
                                            CommaToken(
                                                TokenIdx(
                                                    162,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    166,
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
                                                167,
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
                                                            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
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
                                                                161,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                165,
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 254,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            159,
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
                                                                    value: 255,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            163,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Type(
                        TypeNodeDecl::PropsStruct(
                            PropsStructTypeNodeDecl {
                                node_path: TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 83,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: Ok(
                                    PropsStructLeftCurlyBrace(
                                        LeftCurlyBraceToken(
                                            TokenIdx(
                                                489,
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
                                                                value: 278,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        490,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        491,
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
                                                                value: 239,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        494,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        495,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                        ],
                                        separators: [
                                            CommaToken(
                                                TokenIdx(
                                                    493,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    497,
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
                                                498,
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
                                                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
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
                                                                492,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                496,
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
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentToken {
                                                        ident: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 278,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            490,
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
                                                                    value: 239,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            494,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Type(
                        TypeNodeDecl::PropsStruct(
                            PropsStructTypeNodeDecl {
                                node_path: TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 85,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: Ok(
                                    PropsStructLeftCurlyBrace(
                                        LeftCurlyBraceToken(
                                            TokenIdx(
                                                597,
                                            ),
                                        ),
                                    ),
                                ),
                                fields: Ok(
                                    SeparatedSmallList {
                                        elements: [
                                            PropsFieldDeclPattern {
                                                decorators: [],
                                                visibility: Some(
                                                    Pub,
                                                ),
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 359,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        599,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        600,
                                                    ),
                                                ),
                                                ty_expr_idx: 0,
                                                initialization: None,
                                            },
                                            PropsFieldDeclPattern {
                                                decorators: [],
                                                visibility: Some(
                                                    Pub,
                                                ),
                                                ident_token: IdentToken {
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 360,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        604,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        605,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                        ],
                                        separators: [
                                            CommaToken(
                                                TokenIdx(
                                                    602,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    607,
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
                                                608,
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
                                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                            ident: `ClosedRange`,
                                                            token_idx: TokenIdx(
                                                                601,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `ClosedRange`,
                                                            token_idx: TokenIdx(
                                                                606,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                    value: 359,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            599,
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
                                                                    value: 360,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            604,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Type(
                        TypeNodeDecl::PropsStruct(
                            PropsStructTypeNodeDecl {
                                node_path: TypeNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 87,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: Ok(
                                    PropsStructLeftCurlyBrace(
                                        LeftCurlyBraceToken(
                                            TokenIdx(
                                                733,
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
                                                                value: 359,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        734,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        735,
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
                                                                value: 360,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        738,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        739,
                                                    ),
                                                ),
                                                ty_expr_idx: 1,
                                                initialization: None,
                                            },
                                        ],
                                        separators: [
                                            CommaToken(
                                                TokenIdx(
                                                    737,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    741,
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
                                                742,
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
                                                            path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                            ident: `ClosedRange`,
                                                            token_idx: TokenIdx(
                                                                736,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `ClosedRange`,
                                                            token_idx: TokenIdx(
                                                                740,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                    value: 359,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            734,
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
                                                                    value: 360,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            738,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ImplBlock(
                    ImplBlockNodePath::TypeImplBlock(
                        TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ImplBlock(
                    ImplBlockNodeDecl::Type(
                        TypeImplBlockNodeDecl {
                            node_path: TypeImplBlockNodePath {
                                path: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 79,
                            impl_block: TypeImplBlockNode {
                                node_path: TypeImplBlockNodePath {
                                    path: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 79,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                ty_expr: 6,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        5..10,
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            ty_expr: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TypeImplBlock(
                                                TypeImplBlockNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `Point2d`,
                                                        token_idx: TokenIdx(
                                                            14,
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
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `from_i_shift28`,
                                    item_kind: AssociatedFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::AssociatedFn(
                            TypeAssociatedFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `from_i_shift28`,
                                            item_kind: AssociatedFn,
                                        },
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 5,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    20,
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
                                                    20,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Point2d`,
                                                                        token_idx: TokenIdx(
                                                                            14,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `from_i_shift28`,
                                                                item_kind: AssociatedFn,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `vector`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `vector`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `vector`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 6,
                                    ident: `vector`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 6,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                51,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                52,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                53,
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
                                                55,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Point2d`,
                                                                        token_idx: TokenIdx(
                                                                            14,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `vector`,
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
                                                                54,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `to`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `to`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `to`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 7,
                                    ident: `to`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 7,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    69,
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
                                                    69,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Point2d`,
                                                                        token_idx: TokenIdx(
                                                                            14,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `to`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `norm`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `norm`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `norm`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 8,
                                    ident: `norm`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 8,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                96,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                97,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                98,
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
                                                100,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Point2d`,
                                                                        token_idx: TokenIdx(
                                                                            14,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `norm`,
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
                                                                99,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `dist`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `dist`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `dist`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 9,
                                    ident: `dist`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 9,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    125,
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
                                                    125,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ],
                                                    },
                                                    principal_entity_path_expr_arena: Arena {
                                                        data: [
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameToken::Ident(
                                                                    IdentToken {
                                                                        ident: `Point2d`,
                                                                        token_idx: TokenIdx(
                                                                            14,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `dist`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ImplBlock(
                    ImplBlockNodePath::TypeImplBlock(
                        TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ImplBlock(
                    ImplBlockNodeDecl::Type(
                        TypeImplBlockNodeDecl {
                            node_path: TypeImplBlockNodePath {
                                path: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 82,
                            impl_block: TypeImplBlockNode {
                                node_path: TypeImplBlockNodePath {
                                    path: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 82,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        168,
                                    ),
                                },
                                ty_expr: 7,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        41..49,
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    168,
                                ),
                            },
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            ty_expr: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            170,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TypeImplBlock(
                                                TypeImplBlockNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            169,
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
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `point`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `point`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `point`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 41,
                                    ident: `point`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 41,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                174,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                175,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                176,
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
                                                178,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `point`,
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
                                                                177,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `to`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `to`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `to`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 42,
                                    ident: `to`,
                                    item_kind: MethodFn,
                                    visibility: Scope::Pub,
                                    is_generic: false,
                                },
                                ast_idx: 42,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    193,
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
                                                    193,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `to`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `norm`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `norm`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `norm`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 43,
                                    ident: `norm`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 43,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                220,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                221,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                222,
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
                                                224,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `norm`,
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
                                                                223,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `dot`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `dot`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `dot`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 44,
                                    ident: `dot`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 44,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    249,
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
                                                    249,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `dot`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `cross`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `cross`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `cross`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 45,
                                    ident: `cross`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 45,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    274,
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
                                                    274,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `cross`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `angle`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `angle`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `angle`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 46,
                                    ident: `angle`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 46,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    299,
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
                                                    299,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `angle`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `rotation_direction_to`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `rotation_direction_to`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `rotation_direction_to`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 47,
                                    ident: `rotation_direction_to`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 47,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    368,
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
                                                    368,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `rotation_direction_to`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `angle_to`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `angle_to`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `angle_to`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 48,
                                    ident: `angle_to`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 48,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    388,
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
                                                    388,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                            169,
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `angle_to`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ImplBlock(
                    ImplBlockNodePath::TypeImplBlock(
                        TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ImplBlock(
                    ImplBlockNodeDecl::Type(
                        TypeImplBlockNodeDecl {
                            node_path: TypeImplBlockNodePath {
                                path: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 84,
                            impl_block: TypeImplBlockNode {
                                node_path: TypeImplBlockNodePath {
                                    path: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 84,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        499,
                                    ),
                                },
                                ty_expr: 8,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        56..58,
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    499,
                                ),
                            },
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            ty_expr: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            501,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TypeImplBlock(
                                                TypeImplBlockNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                        ident: `ClosedRange`,
                                                        token_idx: TokenIdx(
                                                            500,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `relative_range`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `relative_range`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `relative_range`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 56,
                                    ident: `relative_range`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 56,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    505,
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
                                                    505,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                        ident: `ClosedRange`,
                                                                        token_idx: TokenIdx(
                                                                            500,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `relative_range`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `relative_point`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `relative_point`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `relative_point`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 57,
                                    ident: `relative_point`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 57,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    568,
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
                                                    568,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                        ident: `ClosedRange`,
                                                                        token_idx: TokenIdx(
                                                                            500,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `relative_point`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ImplBlock(
                    ImplBlockNodePath::TypeImplBlock(
                        TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ImplBlock(
                    ImplBlockNodeDecl::Type(
                        TypeImplBlockNodeDecl {
                            node_path: TypeImplBlockNodePath {
                                path: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 86,
                            impl_block: TypeImplBlockNode {
                                node_path: TypeImplBlockNodePath {
                                    path: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 86,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        609,
                                    ),
                                },
                                ty_expr: 9,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        64..70,
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    609,
                                ),
                            },
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            ty_expr: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            611,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TypeImplBlock(
                                                TypeImplBlockNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                        ident: `BoundingBox`,
                                                        token_idx: TokenIdx(
                                                            610,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `relative_bounding_box`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `relative_bounding_box`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `relative_bounding_box`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 64,
                                    ident: `relative_bounding_box`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 64,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    615,
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
                                                    615,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        ident: `BoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            610,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `relative_bounding_box`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `relative_point`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `relative_point`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `relative_point`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 65,
                                    ident: `relative_point`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 65,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    650,
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
                                                    650,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        ident: `BoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            610,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `relative_point`,
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
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `xmin`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `xmin`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `xmin`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 66,
                                    ident: `xmin`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 66,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                684,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                685,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                686,
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
                                                688,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        ident: `BoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            610,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `xmin`,
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
                                                                687,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `xmax`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `xmax`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `xmax`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 67,
                                    ident: `xmax`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 67,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                696,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                697,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                698,
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
                                                700,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        ident: `BoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            610,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `xmax`,
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
                                                                699,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `ymin`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `ymin`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `ymin`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 68,
                                    ident: `ymin`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 68,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                708,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                709,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                710,
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
                                                712,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        ident: `BoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            610,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `ymin`,
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
                                                                711,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `ymax`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `ymax`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `ymax`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 69,
                                    ident: `ymax`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 69,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                720,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                721,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                722,
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
                                                724,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        ident: `BoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            610,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `ymax`,
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
                                                                723,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::ImplBlock(
                    ImplBlockNodePath::TypeImplBlock(
                        TypeImplBlockNodePath {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ImplBlock(
                    ImplBlockNodeDecl::Type(
                        TypeImplBlockNodeDecl {
                            node_path: TypeImplBlockNodePath {
                                path: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 88,
                            impl_block: TypeImplBlockNode {
                                node_path: TypeImplBlockNodePath {
                                    path: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 88,
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        743,
                                    ),
                                },
                                ty_expr: 10,
                                items: TypeItems {
                                    ast_idx_range: ArenaIdxRange(
                                        74..78,
                                    ),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    743,
                                ),
                            },
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            ty_expr: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            745,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntityNodePath::ImplBlock(
                                            ImplBlockNodePath::TypeImplBlock(
                                                TypeImplBlockNodePath {
                                                    path: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                        ident: `RelativeBoundingBox`,
                                                        token_idx: TokenIdx(
                                                            744,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `xmin`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `xmin`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `xmin`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 74,
                                    ident: `xmin`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 74,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                748,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                749,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                750,
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
                                                752,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                        ident: `RelativeBoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            744,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `xmin`,
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
                                                                751,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `xmax`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `xmax`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `xmax`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 75,
                                    ident: `xmax`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 75,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                760,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                761,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                762,
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
                                                764,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                        ident: `RelativeBoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            744,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `xmax`,
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
                                                                763,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `ymin`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `ymin`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `ymin`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 76,
                                    ident: `ymin`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 76,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                772,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                773,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                774,
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
                                                776,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                        ident: `RelativeBoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            744,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `ymin`,
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
                                                                775,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `ymax`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::AssociatedItem(
                    AssociatedItemNodeDecl::TypeItem(
                        TypeItemNodeDecl::MethodFn(
                            TypeMethodFnNodeDecl {
                                node_path: TypeItemNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::geom2d`,
                                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `ymax`,
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
                                                    module_path: `mnist_classifier::geom2d`,
                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `ymax`,
                                                item_kind: MethodFn,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                    ast_idx: 77,
                                    ident: `ymax`,
                                    item_kind: MethodFn,
                                    visibility: Scope::PubUnder(
                                        `mnist_classifier::geom2d`,
                                    ),
                                    is_generic: false,
                                },
                                ast_idx: 77,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Ok(
                                    SelfParameterAndExplicitParameters {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                784,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        explicit_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                785,
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    Some(
                                        CurryToken(
                                            TokenIdx(
                                                786,
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
                                                788,
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
                                                                        module_path: `mnist_classifier::geom2d`,
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                        ident: `RelativeBoundingBox`,
                                                                        token_idx: TokenIdx(
                                                                            744,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `ymax`,
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
                                                                787,
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
                                                kind: ReturnType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)