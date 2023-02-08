Ok(
    DefnSheet {
        defns: [
            Type(
                RegularStruct(
                    RegularStructTypeDefn {
                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        decl: RegularStructTypeDecl {
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
                                    9,
                                ),
                            },
                            fields: [
                                RegularStructFieldPattern {
                                    ident_token: IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 210,
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
                                                    value: 211,
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
                    },
                ),
            ),
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
                                    pattern_ty_constraints: [],
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
            AssociatedItem(
                TypeItem(
                    Method(
                        TypeMethodDefn {
                            path: Some(
                                TypeItemPath {
                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ident: `displacement`,
                                    ty_item_kind: Method,
                                },
                            ),
                            decl: TypeMethodDecl {
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
                                                        pattern_ty_constraints: [],
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
                                            pattern_ty_constraints: [],
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
                                return_ty: Ok(
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
                                                                pattern_ty_constraints: [],
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
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    ExprRoot {
                                                        kind: OutputType,
                                                        expr: 0,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        DefnExprPath::AssociatedItem(
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
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    29,
                                                ),
                                            ),
                                            Expr::Field {
                                                self_expr: 0,
                                                dot_token_idx: TokenIdx(
                                                    30,
                                                ),
                                                ident_token: IdentifierToken {
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
                                                self_expr: 2,
                                                dot_token_idx: TokenIdx(
                                                    36,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 1,
                                                dot_token_idx: TokenIdx(
                                                    32,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `to`,
                                                    token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    34,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    3..4,
                                                ),
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
                                    entity_path_expr_arena: Arena {
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
                                            kind: BlockExpr,
                                            expr: 5,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                5,
                            ),
                        },
                    ),
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Method(
                        TypeMethodDefn {
                            path: Some(
                                TypeItemPath {
                                    ty: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ident: `dist_to_point`,
                                    ty_item_kind: Method,
                                },
                            ),
                            decl: TypeMethodDecl {
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
                                                        pattern_ty_constraints: [],
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
                                                                    value: 355,
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
                                return_ty: Ok(
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
                                                                pattern_ty_constraints: [],
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
                                                                            value: 355,
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
                                                        kind: OutputType,
                                                        expr: 1,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        DefnExprPath::AssociatedItem(
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
                                            Expr::SelfValue(
                                                TokenIdx(
                                                    52,
                                                ),
                                            ),
                                            Expr::MethodCall {
                                                self_expr: 0,
                                                dot_token_idx: TokenIdx(
                                                    53,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    55,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    1..1,
                                                ),
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
                                                self_expr: 2,
                                                dot_token_idx: TokenIdx(
                                                    61,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `pt`,
                                                token_idx: TokenIdx(
                                                    66,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            Expr::MethodCall {
                                                self_expr: 3,
                                                dot_token_idx: TokenIdx(
                                                    63,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `to`,
                                                    token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    65,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    4..5,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    67,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ab`,
                                                token_idx: TokenIdx(
                                                    69,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ap`,
                                                token_idx: TokenIdx(
                                                    73,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 6,
                                                dot_token_idx: TokenIdx(
                                                    70,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `dot`,
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    72,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    7..8,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    76,
                                                ),
                                            ),
                                            Expr::BinaryOpn {
                                                lopd: 8,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    75,
                                                ),
                                                ropd: 9,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ap`,
                                                token_idx: TokenIdx(
                                                    78,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 11,
                                                dot_token_idx: TokenIdx(
                                                    79,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `norm`,
                                                    token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    81,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    12..12,
                                                ),
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
                                                self_expr: 13,
                                                dot_token_idx: TokenIdx(
                                                    89,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                },
                                            },
                                            Expr::InheritedSymbol {
                                                ident: `pt`,
                                                token_idx: TokenIdx(
                                                    94,
                                                ),
                                                inherited_symbol_idx: 0,
                                                inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                            },
                                            Expr::MethodCall {
                                                self_expr: 14,
                                                dot_token_idx: TokenIdx(
                                                    91,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `to`,
                                                    token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    93,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    15..16,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    95,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ab`,
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `bp`,
                                                token_idx: TokenIdx(
                                                    101,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 17,
                                                dot_token_idx: TokenIdx(
                                                    98,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `dot`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    100,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    18..19,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    102,
                                                ),
                                            },
                                            Expr::Literal(
                                                TokenIdx(
                                                    104,
                                                ),
                                            ),
                                            Expr::BinaryOpn {
                                                lopd: 19,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                opr_token_idx: TokenIdx(
                                                    103,
                                                ),
                                                ropd: 20,
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `bp`,
                                                token_idx: TokenIdx(
                                                    106,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 22,
                                                dot_token_idx: TokenIdx(
                                                    107,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `norm`,
                                                    token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    109,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    23..23,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ab`,
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ap`,
                                                token_idx: TokenIdx(
                                                    117,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 24,
                                                dot_token_idx: TokenIdx(
                                                    114,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `cross`,
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    116,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    25..26,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                            Expr::CurrentSymbol {
                                                ident: `ab`,
                                                token_idx: TokenIdx(
                                                    124,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            Expr::MethodCall {
                                                self_expr: 26,
                                                dot_token_idx: TokenIdx(
                                                    119,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `abs`,
                                                    token_idx: TokenIdx(
                                                        120,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    121,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    27..27,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    122,
                                                ),
                                            },
                                            Expr::MethodCall {
                                                self_expr: 27,
                                                dot_token_idx: TokenIdx(
                                                    125,
                                                ),
                                                ident_token: IdentifierToken {
                                                    ident: `norm`,
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                                implicit_arguments: None,
                                                lpar_token_idx: TokenIdx(
                                                    127,
                                                ),
                                                nonself_arguments: ArenaIdxRange(
                                                    28..28,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            Expr::BinaryOpn {
                                                lopd: 28,
                                                opr: PureClosed(
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
                                    entity_path_expr_arena: Arena {
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
                                                let_variable_pattern: Ok(
                                                    LetVariablesPattern {
                                                        pattern: 2,
                                                        variables: ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                        colon_token: Ok(
                                                            None,
                                                        ),
                                                        ty: None,
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            87,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    16,
                                                ),
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
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                105,
                                                            ),
                                                        },
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
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    112,
                                                                ),
                                                            },
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
                                                let_variable_pattern: Ok(
                                                    LetVariablesPattern {
                                                        pattern: 0,
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
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    1,
                                                ),
                                            },
                                            Stmt::Let {
                                                let_token: LetToken {
                                                    token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                },
                                                let_variable_pattern: Ok(
                                                    LetVariablesPattern {
                                                        pattern: 1,
                                                        variables: ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                        colon_token: Ok(
                                                            None,
                                                        ),
                                                        ty: None,
                                                    },
                                                ),
                                                assign_token: Ok(
                                                    AssignToken {
                                                        token_idx: TokenIdx(
                                                            59,
                                                        ),
                                                    },
                                                ),
                                                initial_value: Ok(
                                                    5,
                                                ),
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
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                77,
                                                            ),
                                                        },
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
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    84,
                                                                ),
                                                            },
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
                                            data: [
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `ab`,
                                                        token_idx: TokenIdx(
                                                            50,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `ap`,
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `bp`,
                                                        token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                            ],
                                        },
                                        pattern_infos: [
                                            Let,
                                            Let,
                                            Let,
                                        ],
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 356,
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
                                                                value: 357,
                                                            },
                                                        ),
                                                    ),
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    Identifier(
                                                        Word(
                                                            Id {
                                                                value: 358,
                                                            },
                                                        ),
                                                    ),
                                                    2,
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
                                                PatternSymbol::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                    },
                                    symbol_region: SymbolRegion {
                                        inherited_symbol_arena: Arena {
                                            data: [
                                                InheritedSymbol {
                                                    ident: `pt`,
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    kind: InheritedSymbolKind::RegularParameter,
                                                },
                                            ],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `ab`,
                                                    access_start: TokenIdx(
                                                        51,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                129,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `ap`,
                                                    access_start: TokenIdx(
                                                        59,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                129,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `bp`,
                                                    access_start: TokenIdx(
                                                        87,
                                                    ),
                                                    access_end: Some(
                                                        TokenIdxRangeEnd(
                                                            TokenIdx(
                                                                129,
                                                            ),
                                                        ),
                                                    ),
                                                    variant: CurrentSymbolVariant::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: BlockExpr,
                                            expr: 31,
                                        },
                                    ],
                                },
                            },
                            body: Ok(
                                31,
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)