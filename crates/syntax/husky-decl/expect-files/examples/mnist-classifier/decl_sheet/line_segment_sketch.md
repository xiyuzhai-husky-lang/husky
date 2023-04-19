Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                    ),
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectRightCurlyBrace(
                                TokenIdx(
                                    53,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ast_idx: 171,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        171,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 2,
                                                    argument: 3,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                    ident: `LineSegmentStroke`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                kind: FieldType,
                                                expr: 1,
                                            },
                                            ExprRoot {
                                                kind: FieldType,
                                                expr: 4,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        162,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        FieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `contour`,
                                                token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    164,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        FieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `strokes`,
                                                token_idx: TokenIdx(
                                                    168,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    169,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                167,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                173,
                                            ),
                                        ),
                                    ],
                                ),
                                rcurl: RightCurlyBraceToken(
                                    TokenIdx(
                                        174,
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                ast_idx: 173,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
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
                                                        352,
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
                                                        356,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        359,
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
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `u`,
                                                            token_idx: TokenIdx(
                                                                350,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `r`,
                                                            token_idx: TokenIdx(
                                                                354,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
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
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `u`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `r`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
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
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            351,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `u`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            355,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `r`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 0,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
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
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            349,
                                        ),
                                    ),
                                    self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    351,
                                                ),
                                            ),
                                            ty: 0,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    355,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                353,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            357,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            358,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 2,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            360,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                ast_idx: 174,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
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
                                                        449,
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
                                                        453,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        456,
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
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `u`,
                                                            token_idx: TokenIdx(
                                                                447,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `r`,
                                                            token_idx: TokenIdx(
                                                                451,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
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
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `u`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `r`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
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
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            448,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `u`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            452,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `r`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 0,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
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
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            446,
                                        ),
                                    ),
                                    self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    448,
                                                ),
                                            ),
                                            ty: 0,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    452,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                450,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            454,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            455,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 2,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            457,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                ast_idx: 175,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        546,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
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
                                                        547,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        551,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        555,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        558,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `ct`,
                                                            token_idx: TokenIdx(
                                                                544,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                549,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `r`,
                                                            token_idx: TokenIdx(
                                                                553,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
                                                Parameter,
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
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `ct`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `start`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `r`,
                                                        2,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                    Pure,
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
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            545,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `ct`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            550,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `start`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            554,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `r`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 2,
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr: 4,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            543,
                                        ),
                                    ),
                                    self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    545,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    550,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    554,
                                                ),
                                            ),
                                            ty: 3,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                548,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                552,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            556,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            557,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 4,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            559,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                ast_idx: 176,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        779,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
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
                                                        780,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        784,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        788,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        792,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        795,
                                                    ),
                                                    ident: `i32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `ct`,
                                                            token_idx: TokenIdx(
                                                                777,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `start0`,
                                                            token_idx: TokenIdx(
                                                                782,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                786,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `r`,
                                                            token_idx: TokenIdx(
                                                                790,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
                                                Parameter,
                                                Parameter,
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
                                                    PatternSymbol::Atom(
                                                        3,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `ct`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `start0`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `end`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `r`,
                                                        3,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                    Pure,
                                                    Pure,
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
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            778,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `ct`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            783,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `start0`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            787,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `end`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            791,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `r`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 2,
                                                        ty: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 3,
                                                        ty: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr: 5,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            776,
                                        ),
                                    ),
                                    self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    778,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    783,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    787,
                                                ),
                                            ),
                                            ty: 3,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 3,
                                            variables: ArenaIdxRange(
                                                3..4,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    791,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                781,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                785,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                789,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            793,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            794,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 5,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            796,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                ast_idx: 177,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        1045,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        1053,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        1054,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 3,
                                                    argument: 4,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        1046,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        1050,
                                                    ),
                                                    ident: `f32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        1055,
                                                    ),
                                                    ident: `LineSegmentStroke`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `ct`,
                                                            token_idx: TokenIdx(
                                                                1043,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `r`,
                                                            token_idx: TokenIdx(
                                                                1048,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                                Parameter,
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
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `ct`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `r`,
                                                        1,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Pure,
                                                    Pure,
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
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            1044,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `ct`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            1049,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `r`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 0,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 1,
                                                        ty: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr: 5,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            1042,
                                        ),
                                    ),
                                    self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    1044,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    1049,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                1047,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            1051,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            1052,
                                        ),
                                    ),
                                ),
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 5,
                                    },
                                ),
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            1056,
                                        ),
                                    },
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::ImplBlock(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::Impl(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 170,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 170,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            81,
                                        ),
                                    },
                                    ty_expr: 6,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                3..7,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        81,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            83,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                        82,
                                                    ),
                                                    ident: `LineSegmentStroke`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `new`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::AssociatedFn(
                                TypeAssociatedFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `new`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            ident: `new`,
                                            item_kind: AssociatedFn,
                                        },
                                    ),
                                    ast_idx: 3,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                                        82,
                                                                    ),
                                                                    ident: `LineSegmentStroke`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `new`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 3,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                            91,
                                                        ),
                                                        ident: `RawContour`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                        ident: `LineSegmentStroke`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `ct`,
                                                                token_idx: TokenIdx(
                                                                    88,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `from`,
                                                                token_idx: TokenIdx(
                                                                    93,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `to`,
                                                                token_idx: TokenIdx(
                                                                    97,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Pure,
                                                        Pure,
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                    Parameter,
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
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `ct`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `from`,
                                                            1,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `to`,
                                                            2,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Pure,
                                                        Pure,
                                                        Pure,
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
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                89,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `ct`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                94,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `from`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                98,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `to`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 1,
                                                            ty: 2,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 2,
                                                            ty: 3,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                87,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        89,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                            RegularParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        94,
                                                    ),
                                                ),
                                                ty: 2,
                                            },
                                            RegularParameterDeclPattern {
                                                pattern: 2,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        98,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    92,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    96,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                100,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                101,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 4,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                103,
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
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `displacement`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MethodFn(
                                TypeMethodFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `displacement`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            ident: `displacement`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `displacement`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 170,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                },
                                                ty_expr: 6,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            3..7,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 4,
                                        ident: `displacement`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 4,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                                        82,
                                                                    ),
                                                                    ident: `LineSegmentStroke`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `displacement`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
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
                                                            128,
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                125,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                126,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                127,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                129,
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
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::Impl(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 172,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 172,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            175,
                                        ),
                                    },
                                    ty_expr: 7,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                21..26,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        175,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            177,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                        176,
                                                    ),
                                                    ident: `LineSegmentSketch`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `concave_components`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            ident: `concave_components`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `concave_components`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ident: `concave_components`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 172,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                },
                                                ty_expr: 7,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            21..26,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 21,
                                        ident: `concave_components`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 21,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                180,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 2,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            184,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    185,
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
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                                        176,
                                                                    ),
                                                                    ident: `LineSegmentSketch`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `concave_components`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            181,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            182,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 0,
                                                        argument: 1,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            183,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                                    kind: VarType,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `bounding_box`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            ident: `bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `bounding_box`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ident: `bounding_box`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 172,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                },
                                                ty_expr: 7,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            21..26,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 22,
                                        ident: `bounding_box`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::line_segment_sketch`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 22,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                192,
                                            ),
                                        ),
                                    ),
                                    memo_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            194,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    195,
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
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                                        176,
                                                                    ),
                                                                    ident: `LineSegmentSketch`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `bounding_box`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
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
                                                            193,
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
            ),
            (
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `new`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::AssociatedFn(
                                TypeAssociatedFnDecl {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `new`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            ident: `new`,
                                            item_kind: AssociatedFn,
                                        },
                                    ),
                                    ast_idx: 23,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::Type(
                                                                    TypeImplBlockId {
                                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                                        176,
                                                                    ),
                                                                    ident: `LineSegmentSketch`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                                expr: 0,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `new`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            321,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                            322,
                                                        ),
                                                        ident: `RawContour`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            326,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            329,
                                                        ),
                                                        ident: `LineSegmentSketch`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `ct`,
                                                                token_idx: TokenIdx(
                                                                    319,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `r`,
                                                                token_idx: TokenIdx(
                                                                    324,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Pure,
                                                        Pure,
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
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
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `ct`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `r`,
                                                            1,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_modifiers: ArenaMap {
                                                    data: [
                                                        Pure,
                                                        Pure,
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
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                320,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `ct`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                325,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `r`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitParameter {
                                                            pattern_expr: 1,
                                                            ty: 2,
                                                        },
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                318,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        320,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                            RegularParameterDeclPattern {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        325,
                                                    ),
                                                ),
                                                ty: 2,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    323,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                327,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                328,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                330,
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