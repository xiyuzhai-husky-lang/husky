Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ast_idx: 80,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    6,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    10,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 166,
                                                },
                                            ),
                                        ),
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 167,
                                                },
                                            ),
                                        ),
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
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ast_idx: 82,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    148,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 166,
                                                },
                                            ),
                                        ),
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 167,
                                                },
                                            ),
                                        ),
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
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ast_idx: 83,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    165,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 166,
                                                },
                                            ),
                                        ),
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 167,
                                                },
                                            ),
                                        ),
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
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ast_idx: 85,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    492,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    496,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 191,
                                                },
                                            ),
                                        ),
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 150,
                                                },
                                            ),
                                        ),
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
            Err(
                Expr(
                    ExpectRightCurlyBrace(
                        TokenIdx(
                            598,
                        ),
                    ),
                ),
            ),
            Ok(
                Type(
                    RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ast_idx: 90,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    736,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 192,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 26,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    740,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 192,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 26,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 277,
                                                },
                                            ),
                                        ),
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
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 278,
                                                },
                                            ),
                                        ),
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
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 81,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::geom2d`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    },
                                },
                                ast_idx: 81,
                                body: ArenaIdxRange(
                                    5..10,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 23,
                                        },
                                    ),
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
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 81,
                                                body: ArenaIdxRange(
                                                    5..10,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 23,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    14,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 180,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 23,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 253,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `from_i_shift28`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 81,
                                        body: ArenaIdxRange(
                                            5..10,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 5,
                                    ident: `from_i_shift28`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: Public,
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 81,
                                                                body: ArenaIdxRange(
                                                                    5..10,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 23,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 180,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 23,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 253,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ident: `from_i_shift28`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 81,
                                                        body: ArenaIdxRange(
                                                            5..10,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 5,
                                                    ident: `from_i_shift28`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: Public,
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::i32`, `Alien`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::i32`, `Alien`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 38,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 9,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 38,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 9,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        29,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 144,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                20,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 173,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 144,
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
                                                                    value: 173,
                                                                },
                                                            ),
                                                        ),
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
                                                        0,
                                                    ),
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 144,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            21,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 173,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            25,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
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
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 255,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `vector`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 81,
                                        body: ArenaIdxRange(
                                            5..10,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 6,
                                    ident: `vector`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 81,
                                                                body: ArenaIdxRange(
                                                                    5..10,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 23,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 180,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 23,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 255,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ident: `vector`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 81,
                                                        body: ArenaIdxRange(
                                                            5..10,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 6,
                                                    ident: `vector`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 203,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 81,
                                        body: ArenaIdxRange(
                                            5..10,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 7,
                                    ident: `to`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 81,
                                                                body: ArenaIdxRange(
                                                                    5..10,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 23,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 180,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 23,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 203,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ident: `to`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 81,
                                                        body: ArenaIdxRange(
                                                            5..10,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 7,
                                                    ident: `to`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            70,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 256,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 81,
                                        body: ArenaIdxRange(
                                            5..10,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 8,
                                    ident: `norm`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 81,
                                                                body: ArenaIdxRange(
                                                                    5..10,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 23,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 180,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 23,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 256,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ident: `norm`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 81,
                                                        body: ArenaIdxRange(
                                                            5..10,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 8,
                                                    ident: `norm`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 258,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            ident: `dist`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 81,
                                        body: ArenaIdxRange(
                                            5..10,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 23,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 9,
                                    ident: `dist`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 81,
                                                                body: ArenaIdxRange(
                                                                    5..10,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 23,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    14,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 180,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 23,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 23,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 258,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ident: `dist`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 81,
                                                        body: ArenaIdxRange(
                                                            5..10,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 9,
                                                    ident: `dist`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        127,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            126,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 84,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::geom2d`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    },
                                },
                                ast_idx: 84,
                                body: ArenaIdxRange(
                                    41..49,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 25,
                                        },
                                    ),
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
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 84,
                                                body: ArenaIdxRange(
                                                    41..49,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 25,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    169,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 199,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 25,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 190,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `point`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 41,
                                    ident: `point`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: Public,
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 190,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `point`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 41,
                                                    ident: `point`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: Public,
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 203,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 42,
                                    ident: `to`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: Public,
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 203,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `to`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 42,
                                                    ident: `to`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: Public,
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            194,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 256,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 43,
                                    ident: `norm`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 256,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `norm`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 43,
                                                    ident: `norm`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        223,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 260,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `dot`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 44,
                                    ident: `dot`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 260,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `dot`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 44,
                                                    ident: `dot`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        251,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            250,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 261,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `cross`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 45,
                                    ident: `cross`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 261,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `cross`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 45,
                                                    ident: `cross`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        279,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            275,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 262,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `angle`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 46,
                                    ident: `angle`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 262,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `angle`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 46,
                                                    ident: `angle`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::basic::bool`, `Alien`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        301,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 12,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        304,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 263,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 263,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 263,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            300,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 267,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `rotation_direction_to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 47,
                                    ident: `rotation_direction_to`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 267,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `rotation_direction_to`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 47,
                                                    ident: `rotation_direction_to`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::i32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        370,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        373,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 38,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 9,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            369,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 25,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 268,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            ident: `angle_to`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            },
                                        },
                                        ast_idx: 84,
                                        body: ArenaIdxRange(
                                            41..49,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 25,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 48,
                                    ident: `angle_to`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 84,
                                                                body: ArenaIdxRange(
                                                                    41..49,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 25,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    169,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 199,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 25,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 25,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 268,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ident: `angle_to`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 84,
                                                        body: ArenaIdxRange(
                                                            41..49,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 48,
                                                    ident: `angle_to`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::basic::bool`, `Alien`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        390,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 199,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 25,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        394,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 12,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
                                                            token_idx: TokenIdx(
                                                                388,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 263,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
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
                                                                    value: 263,
                                                                },
                                                            ),
                                                        ),
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
                                                        0,
                                                    ),
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            389,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 263,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            393,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
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
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Err(
                ImplBlockErr,
            ),
            Err(
                UnableToParseImplBlockDeclForTyMethodDecl,
            ),
            Err(
                UnableToParseImplBlockDeclForTyMethodDecl,
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 89,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::geom2d`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    },
                                },
                                ast_idx: 89,
                                body: ArenaIdxRange(
                                    66..72,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 27,
                                        },
                                    ),
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
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 89,
                                                body: ArenaIdxRange(
                                                    66..72,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 27,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    610,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 184,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 27,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 193,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 89,
                                        body: ArenaIdxRange(
                                            66..72,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 66,
                                    ident: `relative_bounding_box`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 89,
                                                                body: ArenaIdxRange(
                                                                    66..72,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 27,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    610,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 184,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 27,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 27,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 193,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ident: `relative_bounding_box`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 89,
                                                        body: ArenaIdxRange(
                                                            66..72,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 66,
                                                    ident: `relative_bounding_box`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        617,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 184,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        620,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 194,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 36,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            616,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 276,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `relative_point`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 89,
                                        body: ArenaIdxRange(
                                            66..72,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 67,
                                    ident: `relative_point`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 89,
                                                                body: ArenaIdxRange(
                                                                    66..72,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 27,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    610,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 184,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 27,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 27,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 276,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ident: `relative_point`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 89,
                                                        body: ArenaIdxRange(
                                                            66..72,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 67,
                                                    ident: `relative_point`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                                EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        652,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 180,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 23,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Root {
                                                    token_idx: TokenIdx(
                                                        655,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 259,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 24,
                                                                },
                                                            ),
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
                                                    Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 190,
                                                                    },
                                                                ),
                                                            ),
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
                                                                    value: 190,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    Atom(
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 190,
                                                                },
                                                            ),
                                                        ),
                                                        access_start: TokenIdx(
                                                            651,
                                                        ),
                                                        access_end: None,
                                                        variant: Parameter {
                                                            pattern_symbol: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            ty_constraints: [
                                                RegularParameter {
                                                    pattern: 0,
                                                    ty: 0,
                                                },
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 186,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `xmin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 89,
                                        body: ArenaIdxRange(
                                            66..72,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 68,
                                    ident: `xmin`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 89,
                                                                body: ArenaIdxRange(
                                                                    66..72,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 27,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    610,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 184,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 27,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 27,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 186,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ident: `xmin`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 89,
                                                        body: ArenaIdxRange(
                                                            66..72,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 68,
                                                    ident: `xmin`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        687,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 187,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `xmax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 89,
                                        body: ArenaIdxRange(
                                            66..72,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 69,
                                    ident: `xmax`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 89,
                                                                body: ArenaIdxRange(
                                                                    66..72,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 27,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    610,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 184,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 27,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 27,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 187,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ident: `xmax`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 89,
                                                        body: ArenaIdxRange(
                                                            66..72,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 69,
                                                    ident: `xmax`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        699,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 188,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `ymin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 89,
                                        body: ArenaIdxRange(
                                            66..72,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 70,
                                    ident: `ymin`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 89,
                                                                body: ArenaIdxRange(
                                                                    66..72,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 27,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    610,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 184,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 27,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 27,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 188,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ident: `ymin`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 89,
                                                        body: ArenaIdxRange(
                                                            66..72,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 70,
                                                    ident: `ymin`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        711,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 27,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 189,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            ident: `ymax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 89,
                                        body: ArenaIdxRange(
                                            66..72,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 27,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 71,
                                    ident: `ymax`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 89,
                                                                body: ArenaIdxRange(
                                                                    66..72,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 27,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    610,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 184,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 27,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 27,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 189,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ident: `ymax`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 89,
                                                        body: ArenaIdxRange(
                                                            66..72,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 27,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 71,
                                                    ident: `ymax`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        723,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 91,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::geom2d`,
                                    impl_block_kind: Type {
                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    },
                                },
                                ast_idx: 91,
                                body: ArenaIdxRange(
                                    76..80,
                                ),
                                variant: Type {
                                    ty: TypePath(
                                        Id {
                                            value: 28,
                                        },
                                    ),
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
                                    path: Decl(
                                        ImplBlock(
                                            ImplBlock {
                                                id: ImplBlockId {
                                                    module_path: `mnist_classifier::geom2d`,
                                                    impl_block_kind: Type {
                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    },
                                                },
                                                ast_idx: 91,
                                                body: ArenaIdxRange(
                                                    76..80,
                                                ),
                                                variant: Type {
                                                    ty: TypePath(
                                                        Id {
                                                            value: 28,
                                                        },
                                                    ),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    744,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 194,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 28,
                                                            },
                                                        ),
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
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: Type,
                                            expr: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 186,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `xmin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 91,
                                        body: ArenaIdxRange(
                                            76..80,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 76,
                                    ident: `xmin`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 91,
                                                                body: ArenaIdxRange(
                                                                    76..80,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 28,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    744,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 194,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 28,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 186,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            ident: `xmin`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 91,
                                                        body: ArenaIdxRange(
                                                            76..80,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 76,
                                                    ident: `xmin`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        751,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 187,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `xmax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 91,
                                        body: ArenaIdxRange(
                                            76..80,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 77,
                                    ident: `xmax`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 91,
                                                                body: ArenaIdxRange(
                                                                    76..80,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 28,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    744,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 194,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 28,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 187,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            ident: `xmax`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 91,
                                                        body: ArenaIdxRange(
                                                            76..80,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 77,
                                                    ident: `xmax`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        763,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 188,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `ymin`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 91,
                                        body: ArenaIdxRange(
                                            76..80,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 78,
                                    ident: `ymin`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 91,
                                                                body: ArenaIdxRange(
                                                                    76..80,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 28,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    744,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 194,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 28,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 188,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            ident: `ymin`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 91,
                                                        body: ArenaIdxRange(
                                                            76..80,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 78,
                                                    ident: `ymin`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        775,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
            Ok(
                AssociatedItem(
                    TypeItem(
                        Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: ModulePath(
                                                Id {
                                                    value: 35,
                                                },
                                            ),
                                            impl_block_kind: Type {
                                                ty: TypePath(
                                                    Id {
                                                        value: 28,
                                                    },
                                                ),
                                            },
                                        },
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 189,
                                                },
                                            ),
                                        ),
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            ident: `ymax`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::geom2d`,
                                            impl_block_kind: Type {
                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                            },
                                        },
                                        ast_idx: 91,
                                        body: ArenaIdxRange(
                                            76..80,
                                        ),
                                        variant: Type {
                                            ty: TypePath(
                                                Id {
                                                    value: 28,
                                                },
                                            ),
                                        },
                                    },
                                    ast_idx: 79,
                                    ident: `ymax`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
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
                                                    path: Decl(
                                                        ImplBlock(
                                                            ImplBlock {
                                                                id: ImplBlockId {
                                                                    module_path: `mnist_classifier::geom2d`,
                                                                    impl_block_kind: Type {
                                                                        ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                    },
                                                                },
                                                                ast_idx: 91,
                                                                body: ArenaIdxRange(
                                                                    76..80,
                                                                ),
                                                                variant: Type {
                                                                    ty: TypePath(
                                                                        Id {
                                                                            value: 28,
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            Root {
                                                                token_idx: TokenIdx(
                                                                    744,
                                                                ),
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 194,
                                                                        },
                                                                    ),
                                                                ),
                                                                entity_path: ModuleItem(
                                                                    Type(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 28,
                                                                            },
                                                                        ),
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
                                                        ty_constraints: [],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: Type,
                                                            expr: 0,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: Decl(
                                            AssociatedItem(
                                                AssociatedItem {
                                                    id: AssociatedItemId {
                                                        impl_block_id: ImplBlockId {
                                                            module_path: ModulePath(
                                                                Id {
                                                                    value: 35,
                                                                },
                                                            ),
                                                            impl_block_kind: Type {
                                                                ty: TypePath(
                                                                    Id {
                                                                        value: 28,
                                                                    },
                                                                ),
                                                            },
                                                        },
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 189,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    path: Some(
                                                        TypeItemPath {
                                                            ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            ident: `ymax`,
                                                            ty_item_kind: Method,
                                                        },
                                                    ),
                                                    impl_block: ImplBlock {
                                                        id: ImplBlockId {
                                                            module_path: `mnist_classifier::geom2d`,
                                                            impl_block_kind: Type {
                                                                ty: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                            },
                                                        },
                                                        ast_idx: 91,
                                                        body: ArenaIdxRange(
                                                            76..80,
                                                        ),
                                                        variant: Type {
                                                            ty: TypePath(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        },
                                                    },
                                                    ast_idx: 79,
                                                    ident: `ymax`,
                                                    associated_item_kind: TypeItem(
                                                        Method,
                                                    ),
                                                    accessibility: PubicUnder(
                                                        `mnist_classifier::geom2d`,
                                                    ),
                                                    is_generic: false,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                Root {
                                                    token_idx: TokenIdx(
                                                        787,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 42,
                                                            },
                                                        ),
                                                    ),
                                                    entity_path: ModuleItem(
                                                        Type(
                                                            TypePath(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
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
                                            ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: OutputType,
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
                                output_ty: Ok(
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
        ],
    },
)