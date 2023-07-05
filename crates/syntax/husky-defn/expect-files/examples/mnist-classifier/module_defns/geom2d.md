Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `x`,
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
                                        ident: `y`,
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
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `x`,
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
                                        ident: `y`,
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
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `x`,
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
                                        ident: `y`,
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
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `min`,
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
                                        ident: `max`,
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
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: Some(
                                        FieldVisibilityExpr::Pub,
                                    ),
                                    ident_token: IdentToken {
                                        ident: `xrange`,
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
                                        FieldVisibilityExpr::Pub,
                                    ),
                                    ident_token: IdentToken {
                                        ident: `yrange`,
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
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::PropsStruct(
                    PropsStructTypeDefn {
                        path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        decl: PropsStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            implicit_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentToken {
                                        ident: `xrange`,
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
                                        ident: `yrange`,
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
                    },
                ),
            ),
        ),
        Defn::ImplBlock(
            ImplBlockDecl::Type(
                TypeImplBlockDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `vector`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `vector`,
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
                        body: Some(
                            6,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                58,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                59,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    60,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                62,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                57,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            61,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                65,
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
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `Vector2d`,
                                                    token_idx: TokenIdx(
                                                        56,
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
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
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
                        body: Some(
                            13,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                102,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                106,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                103,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    104,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                107,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    108,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                110,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                114,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                111,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    112,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 5,
                                            dot_token_idx: TokenIdx(
                                                115,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    116,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                105,
                                            ),
                                            ropd: 3,
                                        },
                                        Expr::Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                113,
                                            ),
                                            ropd: 7,
                                        },
                                        Expr::Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                101,
                                            ),
                                            item: 10,
                                            rpar_token_idx: TokenIdx(
                                                117,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 11,
                                            dot_token_idx: TokenIdx(
                                                118,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `sqrt`,
                                                token_idx: TokenIdx(
                                                    119,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                120,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                121,
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
                                            expr_idx: 12,
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
                                        expr_idx: 12,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 13,
                                    },
                                ],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `point`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `point`,
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
                        body: Some(
                            6,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                181,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                185,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                186,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    187,
                                                ),
                                            },
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                180,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            184,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                188,
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
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `Point2d`,
                                                    token_idx: TokenIdx(
                                                        179,
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
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
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
                        body: Some(
                            13,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                226,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                230,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                227,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    228,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                231,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    232,
                                                ),
                                            },
                                        },
                                        Expr::SelfValue(
                                            TokenIdx(
                                                234,
                                            ),
                                        ),
                                        Expr::SelfValue(
                                            TokenIdx(
                                                238,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 4,
                                            dot_token_idx: TokenIdx(
                                                235,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    236,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 5,
                                            dot_token_idx: TokenIdx(
                                                239,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    240,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                229,
                                            ),
                                            ropd: 3,
                                        },
                                        Expr::Binary {
                                            lopd: 6,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                237,
                                            ),
                                            ropd: 7,
                                        },
                                        Expr::Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                225,
                                            ),
                                            item: 10,
                                            rpar_token_idx: TokenIdx(
                                                241,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 11,
                                            dot_token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `sqrt`,
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                244,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                245,
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
                                            expr_idx: 12,
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
                                        expr_idx: 12,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 13,
                                    },
                                ],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
        Defn::ImplBlock(
            ImplBlockDecl::Type(
                TypeImplBlockDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `xmin`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `xmin`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                689,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                690,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `xrange`,
                                                token_idx: TokenIdx(
                                                    691,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                692,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    693,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `xmax`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `xmax`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                701,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                702,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `xrange`,
                                                token_idx: TokenIdx(
                                                    703,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                704,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    705,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `ymin`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `ymin`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                713,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                714,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `yrange`,
                                                token_idx: TokenIdx(
                                                    715,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                716,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    717,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `ymax`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `ymax`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                725,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                726,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `yrange`,
                                                token_idx: TokenIdx(
                                                    727,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                728,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    729,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
                                    },
                                ],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
        Defn::AssociatedItem(
            AssociatedItemDefn::TypeItem(
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `xmin`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `xmin`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                753,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                754,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `xrange`,
                                                token_idx: TokenIdx(
                                                    755,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                756,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    757,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `xmax`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `xmax`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                765,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                766,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `xrange`,
                                                token_idx: TokenIdx(
                                                    767,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                768,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    769,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `ymin`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `ymin`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                777,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                778,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `yrange`,
                                                token_idx: TokenIdx(
                                                    779,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                780,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `min`,
                                                token_idx: TokenIdx(
                                                    781,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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
                TypeItemDefn::MethodFn(
                    TypeMethodFnDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::geom2d`,
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `ymax`,
                            item_kind: MethodFn,
                        },
                        decl: TypeMethodFnDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::geom2d`,
                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `ymax`,
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
                        body: Some(
                            3,
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
                                ),
                                path: RegionPath::Defn(
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
                                        Expr::SelfValue(
                                            TokenIdx(
                                                789,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                790,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `yrange`,
                                                token_idx: TokenIdx(
                                                    791,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 1,
                                            dot_token_idx: TokenIdx(
                                                792,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `max`,
                                                token_idx: TokenIdx(
                                                    793,
                                                ),
                                            },
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
                                            expr_idx: 2,
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
                                        expr_idx: 2,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 3,
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