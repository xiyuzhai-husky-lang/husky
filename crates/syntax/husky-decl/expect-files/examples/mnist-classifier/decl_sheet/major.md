Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                            ast_idx: 23,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        33,
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
                                        37,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    34,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    36,
                                                ),
                                                ident: `ConnectedComponent`,
                                                entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                            ast_idx: 24,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        45,
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
                                        47,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    46,
                                                ),
                                                ident: `ConnectedComponent`,
                                                entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
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
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                            ast_idx: 25,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        95,
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
                                        97,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
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
                                                    96,
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
                                        allow_self_type: False,
                                        allow_self_value: False,
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
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                            ast_idx: 26,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        129,
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
                                        133,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    130,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    131,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                                ident: `RawContour`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                            ast_idx: 27,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        140,
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
                                        142,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                                ident: `RawContour`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
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
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                            ast_idx: 28,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        152,
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
                                        154,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    153,
                                                ),
                                                ident: `LineSegmentSketch`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
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
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                            ast_idx: 29,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        161,
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
                                        165,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    162,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    164,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ],
    },
)