Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    TypeDecl::RegularStruct(
                        RegularStructTypeDecl {
                            path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ast_idx: 16,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    12,
                                                ),
                                                ident: `Point2d`,
                                                entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    16,
                                                ),
                                                ident: `Point2d`,
                                                entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                    9,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 197,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            10,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                    ty: 0,
                                },
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 198,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            14,
                                        ),
                                    },
                                    colon: ColonToken {
                                        token_idx: TokenIdx(
                                            15,
                                        ),
                                    },
                                    ty: 1,
                                },
                            ],
                            separators: [
                                CommaToken {
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                                CommaToken {
                                    token_idx: TokenIdx(
                                        17,
                                    ),
                                },
                            ],
                            rcurl: RightCurlyBraceToken {
                                token_idx: TokenIdx(
                                    18,
                                ),
                            },
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    ImplBlockDecl::TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 17,
                            impl_block: ImplBlock {
                                id: ImplBlockId {
                                    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                    impl_block_kind: ImplBlockKind::Type {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    },
                                },
                                ast_idx: 17,
                                body: ArenaIdxRange(
                                    13..15,
                                ),
                                variant: ImplBlockVariant::Type {
                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                },
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    19,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        21,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::ImplBlock(
                                            ImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                impl_block_kind: ImplBlockKind::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                },
                                            },
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    20,
                                                ),
                                                ident: `LineSegment`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            },
                                        },
                                        ident: `displacement`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ident: `displacement`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            },
                                        },
                                        ast_idx: 17,
                                        body: ArenaIdxRange(
                                            13..15,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        },
                                    },
                                    ast_idx: 13,
                                    ident: `displacement`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ident: `displacement`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 13,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    20,
                                                                ),
                                                                ident: `LineSegment`,
                                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                        path: RegionPath::Decl(
                                            DeclExprPath::AssociatedItem(
                                                AssociatedItemId {
                                                    impl_block_id: ImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        },
                                                    },
                                                    ident: `displacement`,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                    ident: `Vector2d`,
                                                    entity_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                            24,
                                        ),
                                    },
                                    parameters: [],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            26,
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
                                            28,
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
                    AssociatedItemDecl::TypeItem(
                        TypeItemDecl::Method(
                            TypeMethodDecl {
                                associated_item: AssociatedItem {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            },
                                        },
                                        ident: `dist_to_point`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ident: `dist_to_point`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    impl_block: ImplBlock {
                                        id: ImplBlockId {
                                            module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                            impl_block_kind: ImplBlockKind::Type {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            },
                                        },
                                        ast_idx: 17,
                                        body: ArenaIdxRange(
                                            13..15,
                                        ),
                                        variant: ImplBlockVariant::Type {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        },
                                    },
                                    ast_idx: 14,
                                    ident: `dist_to_point`,
                                    associated_item_kind: TypeItem(
                                        Method,
                                    ),
                                    accessibility: PubicUnder(
                                        `mnist_classifier::line_segment_sketch::line_segment`,
                                    ),
                                    is_generic: false,
                                },
                                path: Some(
                                    TypeItemPath {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ident: `dist_to_point`,
                                        ty_item_kind: Method,
                                    },
                                ),
                                ast_idx: 14,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclExprPath::ImplBlock(
                                                            ImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                                impl_block_kind: ImplBlockKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                ),
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    20,
                                                                ),
                                                                ident: `LineSegment`,
                                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                        path: RegionPath::Decl(
                                            DeclExprPath::AssociatedItem(
                                                AssociatedItemId {
                                                    impl_block_id: ImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                        impl_block_kind: ImplBlockKind::Type {
                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                        },
                                                    },
                                                    ident: `dist_to_point`,
                                                },
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        TypePath(`core::num::f32`, `Alien`),
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                    ident: `Point2d`,
                                                    entity_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        47,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: TypePath(`core::num::f32`, `Alien`),
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
                                                            ident: `pt`,
                                                            token_idx: TokenIdx(
                                                                42,
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
                                                                    value: 342,
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
                                                        ident: `pt`,
                                                        access_start: TokenIdx(
                                                            43,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::Parameter {
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
                                            41,
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
                                                    43,
                                                ),
                                            },
                                            ty: 0,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken {
                                        token_idx: TokenIdx(
                                            45,
                                        ),
                                    },
                                },
                                curry_token: Ok(
                                    CurryToken {
                                        token_idx: TokenIdx(
                                            46,
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
                                            48,
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