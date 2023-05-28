Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ast_idx: 203,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Ref,
                                                    opr_token_idx: TokenIdx(
                                                        28,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        29,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        35,
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
                                        25,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `cc`,
                                                token_idx: TokenIdx(
                                                    26,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    27,
                                                ),
                                            ),
                                            ty: 1,
                                            initialization: None,
                                        },
                                        RegularStructFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `points`,
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    32,
                                                ),
                                            ),
                                            ty: 4,
                                            initialization: None,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                30,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                36,
                                            ),
                                        ),
                                    ],
                                ),
                                rcurl: RightCurlyBraceToken(
                                    TokenIdx(
                                        37,
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
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::Enum(
                            EnumTypeDecl {
                                path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                ast_idx: 206,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [],
                                        },
                                        entity_path_expr_arena: Arena {
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
                                implicit_parameter_decl_list: None,
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
                                ast_idx: 207,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        415,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        419,
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
                                                        422,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                            ident: `row`,
                                                            token_idx: TokenIdx(
                                                                413,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `j`,
                                                            token_idx: TokenIdx(
                                                                417,
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
                                                        `row`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `j`,
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
                                                            414,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            418,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `j`,
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
                                            412,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    414,
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
                                                    418,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                416,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            420,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            421,
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
                                            423,
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
                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
                                ast_idx: 208,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        440,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        444,
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
                                                        447,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                            ident: `row`,
                                                            token_idx: TokenIdx(
                                                                438,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `j`,
                                                            token_idx: TokenIdx(
                                                                442,
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
                                                        `row`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `j`,
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
                                                            439,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            443,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `j`,
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
                                            437,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    439,
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
                                                    443,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                441,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            445,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            446,
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
                                            448,
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
                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
                                ast_idx: 209,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        461,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        465,
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
                                                        468,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                            ident: `row`,
                                                            token_idx: TokenIdx(
                                                                459,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `j`,
                                                            token_idx: TokenIdx(
                                                                463,
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
                                                        `row`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `j`,
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
                                                            460,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            464,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `j`,
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
                                            458,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    460,
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
                                                    464,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                462,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            466,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            467,
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
                                            469,
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
                            FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
                                ast_idx: 210,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                        486,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        490,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        494,
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
                                                        498,
                                                    ),
                                                    ident: `Direction`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                            ident: `row_above`,
                                                            token_idx: TokenIdx(
                                                                484,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `row_below`,
                                                            token_idx: TokenIdx(
                                                                488,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `j`,
                                                            token_idx: TokenIdx(
                                                                492,
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
                                                        `row_above`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `row_below`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `j`,
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
                                                            485,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row_above`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            489,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row_below`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            493,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `j`,
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
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 2,
                                                        ty: 2,
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
                                                expr: 3,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            483,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    485,
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
                                                    489,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    493,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                487,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                491,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                495,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            496,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            497,
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
                                            499,
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
                            FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
                                ast_idx: 211,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
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
                                                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        632,
                                                    ),
                                                    ident: `Direction`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        636,
                                                    ),
                                                    ident: `Direction`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        639,
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
                                                            ident: `inward`,
                                                            token_idx: TokenIdx(
                                                                630,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `outward`,
                                                            token_idx: TokenIdx(
                                                                634,
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
                                                        `inward`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `outward`,
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
                                                            631,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `inward`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            635,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `outward`,
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
                                            629,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    631,
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
                                                    635,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                633,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            637,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            638,
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
                                            640,
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
                            FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
                                ast_idx: 212,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
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
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                        696,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        700,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        704,
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
                                                        708,
                                                    ),
                                                    ident: `Direction`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        712,
                                                    ),
                                                    ident: `Direction`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                            ident: `row_above`,
                                                            token_idx: TokenIdx(
                                                                694,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `row_below`,
                                                            token_idx: TokenIdx(
                                                                698,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `j`,
                                                            token_idx: TokenIdx(
                                                                702,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `inward_direction`,
                                                            token_idx: TokenIdx(
                                                                706,
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
                                                        `row_above`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `row_below`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `j`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `inward_direction`,
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
                                                            695,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row_above`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            699,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `row_below`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            703,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `j`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            707,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `inward_direction`,
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
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 2,
                                                        ty: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitParameter {
                                                        pattern_expr: 3,
                                                        ty: 3,
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
                                                expr: 4,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: None,
                                parameter_decl_list: ExplicitParameterDeclList {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            693,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    695,
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
                                                    699,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 2,
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    703,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        RegularParameterDeclPattern {
                                            pattern: 3,
                                            variables: ArenaIdxRange(
                                                3..4,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    707,
                                                ),
                                            ),
                                            ty: 3,
                                        },
                                    ],
                                    commas: [
                                        CommaToken(
                                            TokenIdx(
                                                697,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                701,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                705,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                709,
                                            ),
                                        ),
                                    ],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            710,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            711,
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
                                            713,
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
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                ast_idx: 213,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
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
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        923,
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
                                                        927,
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
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        920,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `prev1`,
                                                token_idx: TokenIdx(
                                                    921,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    922,
                                                ),
                                            ),
                                            ty: 0,
                                            initialization: None,
                                        },
                                        RegularStructFieldDeclPattern {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentToken {
                                                ident: `prev2`,
                                                token_idx: TokenIdx(
                                                    925,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    926,
                                                ),
                                            ),
                                            ty: 1,
                                            initialization: None,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                924,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                928,
                                            ),
                                        ),
                                    ],
                                ),
                                rcurl: RightCurlyBraceToken(
                                    TokenIdx(
                                        929,
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
                            FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                                ast_idx: 214,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        935,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        936,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplication {
                                                    function: 0,
                                                    argument: 1,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
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
                                                        937,
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
                                                        940,
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
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `points`,
                                                            token_idx: TokenIdx(
                                                                933,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSymbol::Atom(
                                                        0,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `points`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
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
                                                            934,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `points`,
                                                            pattern_symbol_idx: 0,
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
                                                        ty: 2,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
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
                                            932,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    934,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            938,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            939,
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
                                            941,
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
                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FugitiveDecl::Fn(
                            FnDecl {
                                path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                ast_idx: 215,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
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
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Ref,
                                                    opr_token_idx: TokenIdx(
                                                        1001,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        1005,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        1006,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        1002,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        1007,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                            ident: `cc`,
                                                            token_idx: TokenIdx(
                                                                999,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Pure,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSymbol::Atom(
                                                        0,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `cc`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
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
                                                            1000,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                            ident: `cc`,
                                                            pattern_symbol_idx: 0,
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
                                            998,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    regular_parameters: [
                                        RegularParameterDeclPattern {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    1000,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            1003,
                                        ),
                                    ),
                                },
                                curry_token: Some(
                                    CurryToken(
                                        TokenIdx(
                                            1004,
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
                                            1008,
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
                    ImplBlockId::TraitForType(
                        TraitForTypeImplBlockId {
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::TraitForType(
                            TraitForTypeImplBlockDecl {
                                ast_idx: 204,
                                impl_block: TraitForTypeImplBlock {
                                    id: TraitForTypeImplBlockId {
                                        module_path: `mnist_classifier::raw_contour`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 204,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            38,
                                        ),
                                    },
                                    trai_expr: 3,
                                    for_token: TokenIdx(
                                        40,
                                    ),
                                    ty_expr: 4,
                                    items: Some(
                                        TraitForType(
                                            TraitForTypeItems {
                                                ast_idx_range: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                        ),
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        38,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                trai_expr: TraitExpr {
                                    expr: 0,
                                },
                                for_token: ConnectionForToken {
                                    token_idx: TokenIdx(
                                        40,
                                    ),
                                },
                                ty_expr: TypeExpr {
                                    expr: 1,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            42,
                                        ),
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::ImplBlock(
                                                ImplBlockId::TraitForType(
                                                    TraitForTypeImplBlockId {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                            ModuleItemPath::Trait(
                                                                TraitPath(`core::visual::Visualize`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        39,
                                                    ),
                                                    ident: `Visualize`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                kind: Trait,
                                                expr: 0,
                                            },
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 1,
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
                        impl_block_id: ImplBlockId::TraitForType(
                            TraitForTypeImplBlockId {
                                module_path: `mnist_classifier::raw_contour`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `visualize`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TraitForTypeItem(
                            TraitForTypeItemDecl::MethodFn(
                                TraitForTypeMethodFnDecl {
                                    path: Some(
                                        TraitForTypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            trai: TraitPath(`core::visual::Visualize`),
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::TraitForType(
                                                TraitForTypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `visualize`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    trai: TraitPath(`core::visual::Visualize`),
                                                    ident: `visualize`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::TraitForType(
                                            TraitForTypeImplBlock {
                                                id: TraitForTypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 204,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                },
                                                trai_expr: 3,
                                                for_token: TokenIdx(
                                                    40,
                                                ),
                                                ty_expr: 4,
                                                items: Some(
                                                    TraitForType(
                                                        TraitForTypeItems {
                                                            ast_idx_range: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        ast_idx: 1,
                                        ident: `visualize`,
                                        associated_item_kind: TraitForTypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::raw_contour`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 1,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::ImplBlock(
                                                                ImplBlockId::TraitForType(
                                                                    TraitForTypeImplBlockId {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::visual::Visualize`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 1,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                        39,
                                                                    ),
                                                                    ident: `Visualize`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                },
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        41,
                                                                    ),
                                                                    ident: `RawContour`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                kind: Trait,
                                                                expr: 0,
                                                            },
                                                            ExprRoot {
                                                                kind: SelfType,
                                                                expr: 1,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                DeclRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::TraitForType(
                                                            TraitForTypeImplBlockId {
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `visualize`,
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
                                                                    TypePath(`core::visual::Html`, `Extern`),
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
                                                            48,
                                                        ),
                                                        ident: `Html`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::visual::Html`, `Extern`),
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
                                                45,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                46,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                47,
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
                                                49,
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
                            module_path: `mnist_classifier::raw_contour`,
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Decl::ImplBlock(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 205,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 205,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            60,
                                        ),
                                    },
                                    ty_expr: 5,
                                    body: Type(
                                        TypeItems {
                                            ast_idx_range: ArenaIdxRange(
                                                29..34,
                                            ),
                                        },
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        60,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            62,
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
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                        61,
                                                    ),
                                                    ident: `RawContour`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `line_segment_sketch`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `line_segment_sketch`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `line_segment_sketch`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ident: `line_segment_sketch`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 205,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                                ty_expr: 5,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            29..34,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 29,
                                        ident: `line_segment_sketch`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::raw_contour`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 29,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                65,
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
                                            67,
                                        ),
                                    ),
                                    expr: None,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                        61,
                                                                    ),
                                                                    ident: `RawContour`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `line_segment_sketch`,
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
                                                            66,
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
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `bounding_box`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `bounding_box`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ident: `bounding_box`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 205,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                                ty_expr: 5,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            29..34,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 30,
                                        ident: `bounding_box`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::raw_contour`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 30,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                78,
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
                                            80,
                                        ),
                                    ),
                                    expr: None,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                        61,
                                                                    ),
                                                                    ident: `RawContour`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                            79,
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
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `relative_bounding_box`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `relative_bounding_box`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ident: `relative_bounding_box`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 205,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                                ty_expr: 5,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            29..34,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 31,
                                        ident: `relative_bounding_box`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::raw_contour`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 31,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                198,
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
                                            200,
                                        ),
                                    ),
                                    expr: None,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                        61,
                                                                    ),
                                                                    ident: `RawContour`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `relative_bounding_box`,
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
                                                            199,
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
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `contour_len`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::MemoizedField(
                                TypeMemoizedFieldDecl {
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `contour_len`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `contour_len`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ident: `contour_len`,
                                                    item_kind: MemoizedField,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 205,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                                ty_expr: 5,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            29..34,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 32,
                                        ident: `contour_len`,
                                        associated_item_kind: TypeItem(
                                            MemoizedField,
                                        ),
                                        visibility: Scope::PubUnder(
                                            `mnist_classifier::raw_contour`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 32,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                220,
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
                                            222,
                                        ),
                                    ),
                                    expr: None,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                        61,
                                                                    ),
                                                                    ident: `RawContour`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `contour_len`,
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
                                                                    TypePath(`core::num::f32`, `Extern`),
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
                                                            221,
                                                        ),
                                                        ident: `f32`,
                                                        entity_path: EntityPath::ModuleItem(
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
                                module_path: `mnist_classifier::raw_contour`,
                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                module_path: `mnist_classifier::raw_contour`,
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `displacement`,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ident: `displacement`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `displacement`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                        ),
                                        impl_block: ImplBlock::Type(
                                            TypeImplBlock {
                                                id: TypeImplBlockId {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 205,
                                                impl_token: ImplToken {
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                                ty_expr: 5,
                                                body: Type(
                                                    TypeItems {
                                                        ast_idx_range: ArenaIdxRange(
                                                            29..34,
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        ast_idx: 33,
                                        ident: `displacement`,
                                        associated_item_kind: TypeItem(
                                            MethodFn,
                                        ),
                                        visibility: Scope::Pub,
                                        is_generic: false,
                                    },
                                    ast_idx: 33,
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
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                        61,
                                                                    ),
                                                                    ident: `RawContour`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                module_path: `mnist_classifier::raw_contour`,
                                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
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
                                                            353,
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
                                                            357,
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
                                                            360,
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
                                                                ident: `start`,
                                                                token_idx: TokenIdx(
                                                                    351,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `end`,
                                                                token_idx: TokenIdx(
                                                                    355,
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
                                                            `start`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `end`,
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
                                                                352,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `start`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                356,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `end`,
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
                                    explicit_parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                350,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        352,
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
                                                        356,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    354,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                358,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                359,
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
                                                361,
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