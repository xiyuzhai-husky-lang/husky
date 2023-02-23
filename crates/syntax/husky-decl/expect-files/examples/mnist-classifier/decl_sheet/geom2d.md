Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                ast_idx: 80,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        6,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        10,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                kind: FieldType,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr: 1,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                                fields: [
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `x`,
                                            token_idx: TokenIdx(
                                                4,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                5,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `y`,
                                            token_idx: TokenIdx(
                                                8,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                9,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                separators: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                ],
                                rcurl: RightCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                ast_idx: 82,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                kind: FieldType,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr: 1,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        145,
                                    ),
                                },
                                fields: [
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `x`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                147,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `y`,
                                            token_idx: TokenIdx(
                                                150,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                151,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                separators: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            149,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            153,
                                        ),
                                    },
                                ],
                                rcurl: RightCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        154,
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ast_idx: 83,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                kind: FieldType,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr: 1,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        158,
                                    ),
                                },
                                fields: [
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `x`,
                                            token_idx: TokenIdx(
                                                159,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                160,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `y`,
                                            token_idx: TokenIdx(
                                                163,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                164,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                separators: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            162,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            166,
                                        ),
                                    },
                                ],
                                rcurl: RightCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        167,
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                ast_idx: 85,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        492,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        496,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Alien`),
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                kind: FieldType,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr: 1,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        489,
                                    ),
                                },
                                fields: [
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `min`,
                                            token_idx: TokenIdx(
                                                490,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                491,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `max`,
                                            token_idx: TokenIdx(
                                                494,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                495,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                separators: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            493,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            497,
                                        ),
                                    },
                                ],
                                rcurl: RightCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        498,
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        ),
                    ),
                ),
                Err(
                    Expr(
                        ExpectRightCurlyBrace(
                            TokenIdx(
                                598,
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                ast_idx: 90,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        736,
                                                    ),
                                                    ident: `ClosedRange`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        740,
                                                    ),
                                                    ident: `ClosedRange`,
                                                    entity_path: EntityPath::ModuleItem(
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                kind: FieldType,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr: 1,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        733,
                                    ),
                                },
                                fields: [
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `xrange`,
                                            token_idx: TokenIdx(
                                                734,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                735,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularStructFieldPattern {
                                        ident_token: IdentifierToken {
                                            ident: `yrange`,
                                            token_idx: TokenIdx(
                                                738,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                739,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                separators: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            737,
                                        ),
                                    },
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            741,
                                        ),
                                    },
                                ],
                                rcurl: RightCurlyBraceToken {
                                    token_idx: TokenIdx(
                                        742,
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TypeImplBlock(
                            TypeImplBlockDecl {
                                ast_idx: 81,
                                impl_block: ImplBlock {
                                    id: ImplBlockId {
                                        module_path: `mnist_classifier::geom2d`,
                                        impl_block_kind: ImplBlockKind::Type {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        },
                                        disambiguator: 0,
                                    },
                                    ast_idx: 81,
                                    body: ArenaIdxRange(
                                        5..10,
                                    ),
                                    variant: ImplBlockVariant::Type {
                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    ident: `Point2d`,
                                                    entity_path: EntityPath::ModuleItem(
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                expr: 0,
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
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `from_i_shift28`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `from_i_shift28`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ident: `from_i_shift28`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 81,
                                            body: ArenaIdxRange(
                                                5..10,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 5,
                                        ident: `from_i_shift28`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::Public,
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `from_i_shift28`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 5,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        14,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `from_i_shift28`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Alien`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            26,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Alien`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `i`,
                                                                token_idx: TokenIdx(
                                                                    20,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `shift`,
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 191,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 219,
                                                                    },
                                                                ),
                                                            ),
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                        PatternSymbol::Atom(
                                                            1,
                                                        ),
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
                                                            ident: `i`,
                                                            access_start: TokenIdx(
                                                                21,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `shift`,
                                                            access_start: TokenIdx(
                                                                25,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                    RegularParameter {
                                                        pattern: 1,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                19,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        21,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                            RegularParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        25,
                                                    ),
                                                },
                                                ty: 1,
                                            },
                                        ],
                                        commas: [
                                            CommaToken {
                                                token_idx: TokenIdx(
                                                    23,
                                                ),
                                            },
                                        ],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                27,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                28,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 2,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                30,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `vector`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `vector`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ident: `vector`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 81,
                                            body: ArenaIdxRange(
                                                5..10,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 6,
                                        ident: `vector`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `vector`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 6,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        14,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `vector`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            54,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                51,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                52,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                53,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                55,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `to`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `to`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ident: `to`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 81,
                                            body: ArenaIdxRange(
                                                5..10,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 7,
                                        ident: `to`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 7,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        14,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `to`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            71,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            74,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    69,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                70,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                68,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                75,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ident: `norm`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 81,
                                            body: ArenaIdxRange(
                                                5..10,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 8,
                                        ident: `norm`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 8,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        14,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                96,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                97,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                98,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `dist`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `dist`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ident: `dist`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 81,
                                            body: ArenaIdxRange(
                                                5..10,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 9,
                                        ident: `dist`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `dist`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 9,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        14,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `dist`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    125,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                126,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                124,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                128,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                129,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                131,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TypeImplBlock(
                            TypeImplBlockDecl {
                                ast_idx: 84,
                                impl_block: ImplBlock {
                                    id: ImplBlockId {
                                        module_path: `mnist_classifier::geom2d`,
                                        impl_block_kind: ImplBlockKind::Type {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        },
                                        disambiguator: 0,
                                    },
                                    ast_idx: 84,
                                    body: ArenaIdxRange(
                                        41..49,
                                    ),
                                    variant: ImplBlockVariant::Type {
                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        168,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            170,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                    ident: `Vector2d`,
                                                    entity_path: EntityPath::ModuleItem(
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                expr: 0,
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
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `point`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `point`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `point`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 41,
                                        ident: `point`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::Public,
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `point`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 41,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `point`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            177,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                174,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                175,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                178,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `to`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `to`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `to`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 42,
                                        ident: `to`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::Public,
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 42,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `to`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            195,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            198,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    193,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                194,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                192,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                196,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                197,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                199,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `norm`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `norm`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `norm`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 43,
                                        ident: `norm`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 43,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            223,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                220,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                221,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                222,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                224,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `dot`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `dot`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `dot`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 44,
                                        ident: `dot`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `dot`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 44,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `dot`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            251,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            254,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    249,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                250,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                248,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                252,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                253,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                255,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `cross`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `cross`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `cross`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 45,
                                        ident: `cross`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `cross`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 45,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `cross`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            276,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            279,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    274,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                275,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                273,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        275,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                277,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                278,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                280,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `angle`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `angle`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `angle`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 46,
                                        ident: `angle`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `angle`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 46,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `angle`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::basic::bool`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            301,
                                                        ),
                                                        ident: `bool`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::basic::bool`, `Alien`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            304,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `is_branch_cut_positive`,
                                                                token_idx: TokenIdx(
                                                                    299,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 309,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `is_branch_cut_positive`,
                                                            access_start: TokenIdx(
                                                                300,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                298,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        300,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                302,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                303,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                305,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `rotation_direction_to`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `rotation_direction_to`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `rotation_direction_to`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 47,
                                        ident: `rotation_direction_to`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `rotation_direction_to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 47,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `rotation_direction_to`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            370,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            373,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Alien`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    368,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                369,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                367,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        369,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                371,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                372,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                374,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `angle_to`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `angle_to`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ident: `angle_to`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 84,
                                            body: ArenaIdxRange(
                                                41..49,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 48,
                                        ident: `angle_to`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `angle_to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 48,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        169,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `angle_to`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::basic::bool`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                        ident: `Vector2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            394,
                                                        ),
                                                        ident: `bool`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::basic::bool`, `Alien`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            397,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    388,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `is_branch_cut_positive`,
                                                                token_idx: TokenIdx(
                                                                    392,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 309,
                                                                    },
                                                                ),
                                                            ),
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
                                                        PatternSymbol::Atom(
                                                            1,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                389,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            ident: `is_branch_cut_positive`,
                                                            access_start: TokenIdx(
                                                                393,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                    RegularParameter {
                                                        pattern: 1,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                387,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        389,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                            RegularParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        393,
                                                    ),
                                                },
                                                ty: 1,
                                            },
                                        ],
                                        commas: [
                                            CommaToken {
                                                token_idx: TokenIdx(
                                                    391,
                                                ),
                                            },
                                        ],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                395,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                396,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 2,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                398,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        impl_block_kind: ImplBlockKind::Err,
                        disambiguator: 0,
                    },
                ),
                Err(
                    ImplBlockErr,
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Err,
                            disambiguator: 0,
                        },
                        ident: `relative_range`,
                    },
                ),
                Err(
                    UnableToParseImplBlockDeclForTyMethodDecl,
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Err,
                            disambiguator: 0,
                        },
                        ident: `relative_point`,
                    },
                ),
                Err(
                    UnableToParseImplBlockDeclForTyMethodDecl,
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TypeImplBlock(
                            TypeImplBlockDecl {
                                ast_idx: 89,
                                impl_block: ImplBlock {
                                    id: ImplBlockId {
                                        module_path: `mnist_classifier::geom2d`,
                                        impl_block_kind: ImplBlockKind::Type {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        },
                                        disambiguator: 0,
                                    },
                                    ast_idx: 89,
                                    body: ArenaIdxRange(
                                        66..72,
                                    ),
                                    variant: ImplBlockVariant::Type {
                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        609,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            611,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        610,
                                                    ),
                                                    ident: `BoundingBox`,
                                                    entity_path: EntityPath::ModuleItem(
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                expr: 0,
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
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `relative_bounding_box`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `relative_bounding_box`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ident: `relative_bounding_box`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 89,
                                            body: ArenaIdxRange(
                                                66..72,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 66,
                                        ident: `relative_bounding_box`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 66,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        610,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `relative_bounding_box`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            617,
                                                        ),
                                                        ident: `BoundingBox`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            620,
                                                        ),
                                                        ident: `RelativeBoundingBox`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `other`,
                                                                token_idx: TokenIdx(
                                                                    615,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `other`,
                                                            access_start: TokenIdx(
                                                                616,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                614,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        616,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                618,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                619,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                621,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `relative_point`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `relative_point`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ident: `relative_point`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 89,
                                            body: ArenaIdxRange(
                                                66..72,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 67,
                                        ident: `relative_point`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `relative_point`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 67,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        610,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `relative_point`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            652,
                                                        ),
                                                        ident: `Point2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            655,
                                                        ),
                                                        ident: `RelativePoint2d`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
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
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `point`,
                                                                token_idx: TokenIdx(
                                                                    650,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 236,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            ident: `point`,
                                                            access_start: TokenIdx(
                                                                651,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                649,
                                            ),
                                        },
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken {
                                                    token_idx: TokenIdx(
                                                        651,
                                                    ),
                                                },
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                653,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                654,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                656,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `xmin`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `xmin`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ident: `xmin`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 89,
                                            body: ArenaIdxRange(
                                                66..72,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 68,
                                        ident: `xmin`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `xmin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 68,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        610,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `xmin`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            687,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                684,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                685,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                686,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                688,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `xmax`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `xmax`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ident: `xmax`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 89,
                                            body: ArenaIdxRange(
                                                66..72,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 69,
                                        ident: `xmax`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `xmax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 69,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        610,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `xmax`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            699,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                696,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                697,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                698,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                700,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `ymin`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `ymin`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ident: `ymin`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 89,
                                            body: ArenaIdxRange(
                                                66..72,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 70,
                                        ident: `ymin`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `ymin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 70,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        610,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `ymin`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            711,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                708,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                709,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                710,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                712,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `ymax`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `ymax`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ident: `ymax`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 89,
                                            body: ArenaIdxRange(
                                                66..72,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 71,
                                        ident: `ymax`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `ymax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 71,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        610,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `ymax`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            723,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                720,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                721,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                722,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                724,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId {
                        module_path: `mnist_classifier::geom2d`,
                        impl_block_kind: ImplBlockKind::Type {
                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TypeImplBlock(
                            TypeImplBlockDecl {
                                ast_idx: 91,
                                impl_block: ImplBlock {
                                    id: ImplBlockId {
                                        module_path: `mnist_classifier::geom2d`,
                                        impl_block_kind: ImplBlockKind::Type {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        },
                                        disambiguator: 0,
                                    },
                                    ast_idx: 91,
                                    body: ArenaIdxRange(
                                        76..80,
                                    ),
                                    variant: ImplBlockVariant::Type {
                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        743,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            745,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: ImplBlockKind::Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        744,
                                                    ),
                                                    ident: `RelativeBoundingBox`,
                                                    entity_path: EntityPath::ModuleItem(
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                expr: 0,
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
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `xmin`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `xmin`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ident: `xmin`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 91,
                                            body: ArenaIdxRange(
                                                76..80,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 76,
                                        ident: `xmin`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `xmin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 76,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        744,
                                                                    ),
                                                                    ident: `RelativeBoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `xmin`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            751,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                748,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                749,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                750,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                752,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `xmax`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `xmax`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ident: `xmax`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 91,
                                            body: ArenaIdxRange(
                                                76..80,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 77,
                                        ident: `xmax`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `xmax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 77,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        744,
                                                                    ),
                                                                    ident: `RelativeBoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `xmax`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            763,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                760,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                761,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                762,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                764,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `ymin`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `ymin`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ident: `ymin`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 91,
                                            body: ArenaIdxRange(
                                                76..80,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 78,
                                        ident: `ymin`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `ymin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 78,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        744,
                                                                    ),
                                                                    ident: `RelativeBoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `ymin`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            775,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                772,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                773,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                774,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                776,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId {
                            module_path: `mnist_classifier::geom2d`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `ymax`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `ymax`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ident: `ymax`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock {
                                            id: ImplBlockId {
                                                module_path: `mnist_classifier::geom2d`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 91,
                                            body: ArenaIdxRange(
                                                76..80,
                                            ),
                                            variant: ImplBlockVariant::Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 79,
                                        ident: `ymax`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::geom2d`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `ymax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 79,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: ImplBlockKind::Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    entity_path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        744,
                                                                    ),
                                                                    ident: `RelativeBoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
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
                                                            pattern_infos: [],
                                                            pattern_symbol_maps: [],
                                                            pattern_symbol_arena: Arena {
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: ImplBlockKind::Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `ymax`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Alien`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            787,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Alien`),
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ParameterDeclList {
                                        lpar: LeftParenthesisToken {
                                            token_idx: TokenIdx(
                                                784,
                                            ),
                                        },
                                        parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken {
                                            token_idx: TokenIdx(
                                                785,
                                            ),
                                        },
                                    },
                                    curry_token: Ok(
                                        CurryToken {
                                            token_idx: TokenIdx(
                                                786,
                                            ),
                                        },
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                788,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)