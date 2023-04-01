Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ast_idx: 120,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                            16,
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
                                                            20,
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
                                                            24,
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
                                                            28,
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
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 2,
                                                },
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            13,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `row_start`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        15,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `row_end`,
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        19,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `upper_mass`,
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        23,
                                                    ),
                                                ),
                                                ty: 2,
                                            },
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `lower_mass`,
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        27,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    17,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    21,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    25,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    29,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            30,
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ast_idx: 121,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                            40,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            37,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            39,
                                                        ),
                                                        opd: 1,
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
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            34,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `matches`,
                                                    token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        36,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    42,
                                                ),
                                            ),
                                        ],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            43,
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                    ast_idx: 122,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
                                                            49,
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
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            50,
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
                                                            54,
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
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `ct`,
                                                                token_idx: TokenIdx(
                                                                    47,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `ct`,
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
                                                            access_start: TokenIdx(
                                                                48,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `ct`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
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
                                                46,
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
                                                        48,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                51,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                52,
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
                                                55,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
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
                                                                    49,
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
                                                            Expr::Prefix {
                                                                opr: Option,
                                                                opr_token_idx: TokenIdx(
                                                                    53,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    50,
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
                                                                    54,
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
                                                            data: [
                                                                PatternExpr::Ident {
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `ct`,
                                                                        token_idx: TokenIdx(
                                                                            47,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `ct`,
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
                                                                    access_start: TokenIdx(
                                                                        48,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `ct`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 1,
                                                            },
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
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `ct`,
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `ct`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 0,
                                                    dot_token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `contour_len`,
                                                        token_idx: TokenIdx(
                                                            61,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `len`,
                                                    token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        65,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 2,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    ropd: 3,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `len`,
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        68,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        67,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        0..3,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            56,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                58,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        4,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 7,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `len`,
                                                            token_idx: TokenIdx(
                                                                57,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `len`,
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
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `ct`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            58,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    69,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `len`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr: 8,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    8,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ast_idx: 123,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::UnrecognizedIdent {
                                                                token_idx: TokenIdx(
                                                                    75,
                                                                ),
                                                                ident: `BinaryImage28`,
                                                            },
                                                        ),
                                                    ),
                                                ],
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
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            72,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            FieldDeclPattern {
                                                decorators: [],
                                                visibility: None,
                                                ident_token: IdentToken {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        74,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        ],
                                        [],
                                    ),
                                    rcurl: RightCurlyBraceToken(
                                        TokenIdx(
                                            76,
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                    ast_idx: 125,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
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
                                                            556,
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
                                                            560,
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
                                                            563,
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
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `a`,
                                                                token_idx: TokenIdx(
                                                                    554,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `x`,
                                                                token_idx: TokenIdx(
                                                                    558,
                                                                ),
                                                            },
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
                                                            `a`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `x`,
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
                                                            access_start: TokenIdx(
                                                                555,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `a`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                559,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `x`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                    ExplicitParameter {
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
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                553,
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
                                                        555,
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
                                                        559,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    557,
                                                ),
                                            ),
                                        ],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                561,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                562,
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
                                                564,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
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
                                                                    556,
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
                                                                    560,
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
                                                                    563,
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
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `a`,
                                                                        token_idx: TokenIdx(
                                                                            554,
                                                                        ),
                                                                    },
                                                                },
                                                                PatternExpr::Ident {
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `x`,
                                                                        token_idx: TokenIdx(
                                                                            558,
                                                                        ),
                                                                    },
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
                                                                    `a`,
                                                                    0,
                                                                ),
                                                            ],
                                                            [
                                                                (
                                                                    `x`,
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
                                                                    access_start: TokenIdx(
                                                                        555,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `a`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        559,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `x`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 0,
                                                            },
                                                            ExplicitParameter {
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
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        575,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `x`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        577,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 0,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        576,
                                                    ),
                                                    ropd: 1,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        572,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `x`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        574,
                                                    ),
                                                    item: 2,
                                                    rpar_token_idx: TokenIdx(
                                                        578,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        581,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `x`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        583,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Shift(
                                                        Shr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        582,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        573,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        580,
                                                    ),
                                                    item: 7,
                                                    rpar_token_idx: TokenIdx(
                                                        584,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        579,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        569,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `a`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        571,
                                                    ),
                                                    item: 10,
                                                    rpar_token_idx: TokenIdx(
                                                        585,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 11,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        570,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        596,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        598,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 14,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                    ropd: 15,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        593,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        595,
                                                    ),
                                                    item: 16,
                                                    rpar_token_idx: TokenIdx(
                                                        599,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        602,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        604,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 19,
                                                    opr: Shift(
                                                        Shr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        603,
                                                    ),
                                                    ropd: 20,
                                                },
                                                Expr::Binary {
                                                    lopd: 17,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        594,
                                                    ),
                                                    ropd: 18,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        601,
                                                    ),
                                                    item: 21,
                                                    rpar_token_idx: TokenIdx(
                                                        605,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 22,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        600,
                                                    ),
                                                    ropd: 23,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        590,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `a`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        592,
                                                    ),
                                                    item: 24,
                                                    rpar_token_idx: TokenIdx(
                                                        606,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 25,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        591,
                                                    ),
                                                    ropd: 26,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `z`,
                                                    token_idx: TokenIdx(
                                                        608,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        610,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Comparison(
                                                        Neq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        609,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        612,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `z`,
                                                    token_idx: TokenIdx(
                                                        614,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 31,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        613,
                                                    ),
                                                    ropd: 32,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        623,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        625,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 34,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        624,
                                                    ),
                                                    ropd: 35,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        620,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        622,
                                                    ),
                                                    item: 36,
                                                    rpar_token_idx: TokenIdx(
                                                        626,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        629,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        631,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 39,
                                                    opr: Shift(
                                                        Shr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        630,
                                                    ),
                                                    ropd: 40,
                                                },
                                                Expr::Binary {
                                                    lopd: 37,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        621,
                                                    ),
                                                    ropd: 38,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        628,
                                                    ),
                                                    item: 41,
                                                    rpar_token_idx: TokenIdx(
                                                        632,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 42,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        627,
                                                    ),
                                                    ropd: 43,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        617,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `a`,
                                                    },
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        619,
                                                    ),
                                                    item: 44,
                                                    rpar_token_idx: TokenIdx(
                                                        633,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `z`,
                                                    token_idx: TokenIdx(
                                                        615,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 45,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        618,
                                                    ),
                                                    ropd: 46,
                                                },
                                                Expr::Binary {
                                                    lopd: 47,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        616,
                                                    ),
                                                    ropd: 48,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        635,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..6,
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
                                                    expr_idx: 33,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 49,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            565,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                568,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        13,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            586,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                589,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        27,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            607,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        30,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    611,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            0..2,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            634,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        50,
                                                    ),
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                567,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `z`,
                                                            token_idx: TokenIdx(
                                                                588,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `y`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `z`,
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
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `a`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `x`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            568,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    636,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `y`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            589,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    636,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `z`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr: 51,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    51,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                    ast_idx: 126,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::UnrecognizedIdent {
                                                                token_idx: TokenIdx(
                                                                    642,
                                                                ),
                                                                ident: `BinaryImage28`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            645,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            646,
                                                        ),
                                                    },
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
                                                    Expr::ExplicitApplication {
                                                        function: 1,
                                                        argument: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            647,
                                                        ),
                                                        ident: `ConnectedComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
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
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `img`,
                                                                token_idx: TokenIdx(
                                                                    640,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `img`,
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
                                                            access_start: TokenIdx(
                                                                641,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
                                                                ident: `img`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
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
                                                639,
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
                                                        641,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                643,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                644,
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
                                                648,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::Err(
                                                                ExprError::Original(
                                                                    OriginalExprError::UnrecognizedIdent {
                                                                        token_idx: TokenIdx(
                                                                            642,
                                                                        ),
                                                                        ident: `BinaryImage28`,
                                                                    },
                                                                ),
                                                            ),
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    645,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    1..1,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    646,
                                                                ),
                                                            },
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
                                                            Expr::ExplicitApplication {
                                                                function: 1,
                                                                argument: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    647,
                                                                ),
                                                                ident: `ConnectedComponent`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    ),
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
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `img`,
                                                                        token_idx: TokenIdx(
                                                                            640,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `img`,
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
                                                                    access_start: TokenIdx(
                                                                        641,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::ExplicitParameter {
                                                                        ident: `img`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 0,
                                                            },
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
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        653,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        654,
                                                    ),
                                                },
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
                                                Expr::ExplicitApplication {
                                                    function: 0,
                                                    argument: 1,
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        657,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        658,
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `img`,
                                                    token_idx: TokenIdx(
                                                        663,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `img`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 4,
                                                    dot_token_idx: TokenIdx(
                                                        664,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `clone`,
                                                        token_idx: TokenIdx(
                                                            665,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        666,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        5..5,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        667,
                                                    ),
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        669,
                                                    ),
                                                    ident: `j`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        6,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        671,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 6,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        670,
                                                    ),
                                                    ropd: 7,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `unsearched`,
                                                    token_idx: TokenIdx(
                                                        674,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        676,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        6,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 9,
                                                    lbox_token_idx: TokenIdx(
                                                        675,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        677,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `unsearched`,
                                                    token_idx: TokenIdx(
                                                        682,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        684,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        6,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 12,
                                                    lbox_token_idx: TokenIdx(
                                                        683,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        685,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        689,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 15,
                                                    dot_token_idx: TokenIdx(
                                                        690,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ctz`,
                                                        token_idx: TokenIdx(
                                                            691,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        692,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        16..16,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        693,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                698,
                                                            ),
                                                            ident: `BinaryImage28`,
                                                        },
                                                    ),
                                                ),
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 17,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        699,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        18..18,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        700,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        701,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        703,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        6,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        710,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `shift`,
                                                    token_idx: TokenIdx(
                                                        712,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        708,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 22,
                                                    opr: Shift(
                                                        Shl,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        711,
                                                    ),
                                                    ropd: 23,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 19,
                                                    lbox_token_idx: TokenIdx(
                                                        702,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        20..21,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        704,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 21,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        707,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        24..26,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            709,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        713,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 26,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        705,
                                                    ),
                                                    ropd: 27,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        718,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        721,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        720,
                                                    ),
                                                    opd: 30,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        723,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        725,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 32,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        724,
                                                    ),
                                                    ropd: 33,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        730,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        6,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        741,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        743,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        745,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 37,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        744,
                                                    ),
                                                    ropd: 38,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 36,
                                                    lbox_token_idx: TokenIdx(
                                                        742,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        39..40,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        746,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `img`,
                                                    token_idx: TokenIdx(
                                                        754,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `img`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        756,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        758,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 43,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        757,
                                                    ),
                                                    ropd: 44,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        761,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        763,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 42,
                                                    lbox_token_idx: TokenIdx(
                                                        755,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        45..46,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        759,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 46,
                                                    lbox_token_idx: TokenIdx(
                                                        762,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        47..48,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        764,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        750,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 41,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        753,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        48..50,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            760,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        765,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 50,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        751,
                                                    ),
                                                    ropd: 51,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        768,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        767,
                                                    ),
                                                    opd: 53,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        772,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        774,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 55,
                                                    opr: Comparison(
                                                        Neq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        773,
                                                    ),
                                                    ropd: 56,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        776,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        778,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 58,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        777,
                                                    ),
                                                    ropd: 59,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        779,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        781,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        783,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 62,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        782,
                                                    ),
                                                    ropd: 63,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 61,
                                                    lbox_token_idx: TokenIdx(
                                                        780,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        64..65,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        784,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        786,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 65,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        785,
                                                    ),
                                                    ropd: 66,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        795,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        797,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 68,
                                                    lbox_token_idx: TokenIdx(
                                                        796,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        69..70,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        798,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `img`,
                                                    token_idx: TokenIdx(
                                                        806,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `img`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        808,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        811,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        813,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        815,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 75,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        814,
                                                    ),
                                                    ropd: 76,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 72,
                                                    lbox_token_idx: TokenIdx(
                                                        807,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        73..74,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        809,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 74,
                                                    lbox_token_idx: TokenIdx(
                                                        812,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        77..78,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        816,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        802,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 71,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        805,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        78..80,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            810,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        817,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 80,
                                                    opr: Closed(
                                                        BitOr,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        803,
                                                    ),
                                                    ropd: 81,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `old_row`,
                                                    token_idx: TokenIdx(
                                                        819,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        821,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 83,
                                                    opr: Comparison(
                                                        Neq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        820,
                                                    ),
                                                    ropd: 84,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `flag`,
                                                    token_idx: TokenIdx(
                                                        823,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        825,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 86,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        824,
                                                    ),
                                                    ropd: 87,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        826,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i`,
                                                    token_idx: TokenIdx(
                                                        828,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 89,
                                                    lbox_token_idx: TokenIdx(
                                                        827,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        90..91,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        829,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `new_row`,
                                                    token_idx: TokenIdx(
                                                        831,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 91,
                                                    opr: Assign,
                                                    opr_token_idx: TokenIdx(
                                                        830,
                                                    ),
                                                    ropd: 92,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `j`,
                                                    token_idx: TokenIdx(
                                                        833,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        6,
                                                    ),
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        835,
                                                    ),
                                                    ident: `k`,
                                                    frame_var_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        95,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 94,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        834,
                                                    ),
                                                    ropd: 95,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        837,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 96,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        836,
                                                    ),
                                                    ropd: 97,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `unsearched`,
                                                    token_idx: TokenIdx(
                                                        839,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `k`,
                                                    token_idx: TokenIdx(
                                                        841,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        95,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        846,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `k`,
                                                    token_idx: TokenIdx(
                                                        848,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        95,
                                                    ),
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 101,
                                                    lbox_token_idx: TokenIdx(
                                                        847,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        102..103,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        849,
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Tilde,
                                                    opr_token_idx: TokenIdx(
                                                        845,
                                                    ),
                                                    opd: 103,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 99,
                                                    lbox_token_idx: TokenIdx(
                                                        840,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        100..101,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        842,
                                                    ),
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        844,
                                                    ),
                                                    item: 104,
                                                    rpar_token_idx: TokenIdx(
                                                        850,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 105,
                                                    opr: AssignClosed(
                                                        BitAnd,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        843,
                                                    ),
                                                    ropd: 106,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `result`,
                                                    token_idx: TokenIdx(
                                                        851,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `mask`,
                                                    token_idx: TokenIdx(
                                                        857,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 109,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        856,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        110..111,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        858,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 108,
                                                    dot_token_idx: TokenIdx(
                                                        852,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            853,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        854,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        111..112,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        859,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `result`,
                                                    token_idx: TokenIdx(
                                                        861,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        26..30,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        655,
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
                                                        706,
                                                    ),
                                                    ident: `horizontal_extend`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        752,
                                                    ),
                                                    ident: `horizontal_extend`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        804,
                                                    ),
                                                    ident: `horizontal_extend`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        855,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Break {
                                                    break_token: BreakToken {
                                                        token_idx: TokenIdx(
                                                            770,
                                                        ),
                                                    },
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 60,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 67,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            738,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 7,
                                                            variables: ArenaIdxRange(
                                                                8..9,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                740,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        40,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            747,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 8,
                                                            variables: ArenaIdxRange(
                                                                9..10,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                749,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        52,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                766,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            54,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        769,
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
                                                    else_branch: None,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                771,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            57,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        775,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..3,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 88,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 93,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            792,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 9,
                                                            variables: ArenaIdxRange(
                                                                10..11,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                794,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        70,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            799,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 10,
                                                            variables: ArenaIdxRange(
                                                                11..12,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                801,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        82,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                818,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            85,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        822,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                7..9,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 34,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            726,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 6,
                                                            variables: ArenaIdxRange(
                                                                7..8,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                729,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        35,
                                                    ),
                                                },
                                                Stmt::ForExt {
                                                    forext_token: ForextToken {
                                                        token_idx: TokenIdx(
                                                            731,
                                                        ),
                                                    },
                                                    eol_colon: Err(
                                                        ExprError::Original(
                                                            OriginalExprError::ExpectEolColon(
                                                                TokenIdx(
                                                                    732,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            3..7,
                                                        ),
                                                    ),
                                                },
                                                Stmt::ForExt {
                                                    forext_token: ForextToken {
                                                        token_idx: TokenIdx(
                                                            787,
                                                        ),
                                                    },
                                                    eol_colon: Err(
                                                        ExprError::Original(
                                                            OriginalExprError::ExpectEolColon(
                                                                TokenIdx(
                                                                    788,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            9..12,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 107,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            679,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 2,
                                                            variables: ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                681,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        14,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            686,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 3,
                                                            variables: ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                688,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        16,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            694,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 4,
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                697,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        18,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 28,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            714,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 5,
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                717,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        29,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            719,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        31,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    722,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            12..16,
                                                        ),
                                                    ),
                                                },
                                                Stmt::ForBetween {
                                                    for_token: StmtForToken {
                                                        token_idx: TokenIdx(
                                                            832,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            835,
                                                        ),
                                                        frame_var_expr_idx: 95,
                                                        frame_var_ident: `k`,
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    94,
                                                                ),
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    97,
                                                                ),
                                                                kind: UpperOpen,
                                                            },
                                                            step: Constant(
                                                                1,
                                                            ),
                                                        },
                                                    },
                                                    frame_var_symbol_idx: 12,
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    838,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            16..17,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 112,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            673,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        11,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    678,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            17..25,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            649,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonToken(
                                                                        TokenIdx(
                                                                            652,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                2,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        EqToken(
                                                            TokenIdx(
                                                                656,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            659,
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
                                                        EqToken(
                                                            TokenIdx(
                                                                662,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::ForBetween {
                                                    for_token: StmtForToken {
                                                        token_idx: TokenIdx(
                                                            668,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            669,
                                                        ),
                                                        frame_var_expr_idx: 6,
                                                        frame_var_ident: `j`,
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: None,
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    7,
                                                                ),
                                                                kind: UpperOpen,
                                                            },
                                                            step: Constant(
                                                                1,
                                                            ),
                                                        },
                                                    },
                                                    frame_var_symbol_idx: 2,
                                                    eol_colon: Ok(
                                                        EolToken::Colon(
                                                            EolColonToken {
                                                                token_idx: TokenIdx(
                                                                    672,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            25..26,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            860,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        113,
                                                    ),
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `result`,
                                                            token_idx: TokenIdx(
                                                                651,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `unsearched`,
                                                            token_idx: TokenIdx(
                                                                661,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                680,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `shift`,
                                                            token_idx: TokenIdx(
                                                                687,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                696,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `flag`,
                                                            token_idx: TokenIdx(
                                                                716,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `i`,
                                                            token_idx: TokenIdx(
                                                                728,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `old_row`,
                                                            token_idx: TokenIdx(
                                                                739,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `new_row`,
                                                            token_idx: TokenIdx(
                                                                748,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `old_row`,
                                                            token_idx: TokenIdx(
                                                                793,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `new_row`,
                                                            token_idx: TokenIdx(
                                                                800,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `result`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `unsearched`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `shift`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `mask`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `flag`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `i`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `old_row`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `new_row`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `old_row`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `new_row`,
                                                        10,
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
                                                    PatternSymbol::Atom(
                                                        3,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        4,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        5,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        6,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        7,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        8,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        9,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        10,
                                                    ),
                                                ],
                                            },
                                        },
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            0,
                                                        ),
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `img`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            652,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    862,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `result`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            662,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    862,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `unsearched`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            673,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    860,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `j`,
                                                            expr_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            681,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    860,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            688,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    860,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `shift`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            697,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    860,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `mask`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            717,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    860,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `flag`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            729,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    832,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `i`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            740,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    787,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `old_row`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            749,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    787,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `new_row`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            794,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    832,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `old_row`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            801,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    832,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `new_row`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            839,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    851,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `k`,
                                                            expr_idx: 95,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                LetVariables {
                                                    pattern: 0,
                                                    ty: 2,
                                                },
                                                FrameVariable,
                                                FrameVariable,
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr: 114,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    114,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplBlockId::Type(
                        TypeImplBlockId {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                Ok(
                    Defn::Impl(
                        ImplBlockDecl::Type(
                            TypeImplBlockDecl {
                                ast_idx: 124,
                                impl_block: TypeImplBlock {
                                    id: TypeImplBlockId {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ast_idx: 124,
                                    impl_token: ImplToken {
                                        token_idx: TokenIdx(
                                            77,
                                        ),
                                    },
                                    ty_expr: 0,
                                    body: ArenaIdxRange(
                                        67..79,
                                    ),
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        77,
                                    ),
                                },
                                implicit_parameter_decl_list: None,
                                ty_expr: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            79,
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
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        78,
                                                    ),
                                                    ident: `ConnectedComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
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
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `raw_contours`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    82,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `eff_holes`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    93,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `max_hole_ilen`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    155,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `max_row_span`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    207,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `row_span_sum`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    245,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `distribution`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    278,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `upper_mass`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    400,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `lower_mass`,
                    },
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    412,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `top_k_row_span_sum`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::connected_component`,
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `top_k_row_span_sum`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::connected_component`,
                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `top_k_row_span_sum`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ident: `top_k_row_span_sum`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `top_k_row_span_sum`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ident: `top_k_row_span_sum`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 124,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            77,
                                                        ),
                                                    },
                                                    ty_expr: 0,
                                                    body: ArenaIdxRange(
                                                        67..79,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 75,
                                            ident: `top_k_row_span_sum`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 75,
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            78,
                                                                        ),
                                                                        ident: `ConnectedComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                            ),
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
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `top_k_row_span_sum`,
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
                                                                427,
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
                                                                430,
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
                                                        data: [
                                                            PatternExpr::Ident {
                                                                modifier: None,
                                                                ident_token: IdentToken {
                                                                    ident: `k`,
                                                                    token_idx: TokenIdx(
                                                                        425,
                                                                    ),
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    pattern_infos: [
                                                        Parameter,
                                                    ],
                                                    pattern_symbol_maps: [
                                                        [
                                                            (
                                                                `k`,
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
                                                                access_start: TokenIdx(
                                                                    426,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ExplicitParameter {
                                                                    ident: `k`,
                                                                    pattern_symbol_idx: 0,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: True,
                                                    pattern_ty_constraints: [
                                                        ExplicitParameter {
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
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    424,
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
                                                            426,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    428,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    429,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 1,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    431,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::connected_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    78,
                                                                                ),
                                                                                ident: `ConnectedComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                                    ),
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
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `top_k_row_span_sum`,
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
                                                                        427,
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
                                                                        430,
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
                                                                data: [
                                                                    PatternExpr::Ident {
                                                                        modifier: None,
                                                                        ident_token: IdentToken {
                                                                            ident: `k`,
                                                                            token_idx: TokenIdx(
                                                                                425,
                                                                            ),
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            pattern_infos: [
                                                                Parameter,
                                                            ],
                                                            pattern_symbol_maps: [
                                                                [
                                                                    (
                                                                        `k`,
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
                                                                        access_start: TokenIdx(
                                                                            426,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                                            ident: `k`,
                                                                            pattern_symbol_idx: 0,
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: True,
                                                            pattern_ty_constraints: [
                                                                ExplicitParameter {
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::connected_component`,
                                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `top_k_row_span_sum`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            436,
                                                        ),
                                                    ),
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            438,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            440,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 1,
                                                        opr: Comparison(
                                                            Greater,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            439,
                                                        ),
                                                        ropd: 2,
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            445,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            452,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            453,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                454,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            456,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 6,
                                                        lbox_token_idx: TokenIdx(
                                                            455,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            7..8,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            457,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            461,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            463,
                                                        ),
                                                        ident: `j`,
                                                        frame_var_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            10,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            465,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            467,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 9,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            462,
                                                        ),
                                                        ropd: 10,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 11,
                                                        opr: Closed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            466,
                                                        ),
                                                        ropd: 12,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 13,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            464,
                                                        ),
                                                        ropd: 14,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            471,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 16,
                                                        dot_token_idx: TokenIdx(
                                                            472,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                473,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            475,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            10,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 17,
                                                        lbox_token_idx: TokenIdx(
                                                            474,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            18..19,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            476,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            469,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 19,
                                                        dot_token_idx: TokenIdx(
                                                            477,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `span`,
                                                            token_idx: TokenIdx(
                                                                478,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            479,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            20..20,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            480,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 20,
                                                        opr: AssignClosed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            470,
                                                        ),
                                                        ropd: 21,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            482,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
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
                                                    Expr::Binary {
                                                        lopd: 23,
                                                        opr: As,
                                                        opr_token_idx: TokenIdx(
                                                            483,
                                                        ),
                                                        ropd: 24,
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            3..9,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            484,
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
                                                data: [
                                                    Stmt::Break {
                                                        break_token: BreakToken {
                                                            token_idx: TokenIdx(
                                                                459,
                                                            ),
                                                        },
                                                    },
                                                    Stmt::IfElse {
                                                        if_branch: IfBranch {
                                                            if_token: IfToken {
                                                                token_idx: TokenIdx(
                                                                    451,
                                                                ),
                                                            },
                                                            condition: Ok(
                                                                8,
                                                            ),
                                                            eol_colon: Ok(
                                                                Colon(
                                                                    EolColonToken {
                                                                        token_idx: TokenIdx(
                                                                            458,
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
                                                        else_branch: None,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 22,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                432,
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
                                                            EqToken(
                                                                TokenIdx(
                                                                    435,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            0,
                                                        ),
                                                    },
                                                    Stmt::Assert {
                                                        assert_token: AssertToken {
                                                            token_idx: TokenIdx(
                                                                437,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            3,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                441,
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
                                                            EqToken(
                                                                TokenIdx(
                                                                    444,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            4,
                                                        ),
                                                    },
                                                    Stmt::ForExt {
                                                        forext_token: ForextToken {
                                                            token_idx: TokenIdx(
                                                                446,
                                                            ),
                                                        },
                                                        eol_colon: Err(
                                                            ExprError::Original(
                                                                OriginalExprError::ExpectEolColon(
                                                                    TokenIdx(
                                                                        447,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                460,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                463,
                                                            ),
                                                            frame_var_expr_idx: 10,
                                                            frame_var_ident: `j`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        9,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        14,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 2,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        468,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                481,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            25,
                                                        ),
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `top_k_row_span_sum`,
                                                                token_idx: TokenIdx(
                                                                    434,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `i`,
                                                                token_idx: TokenIdx(
                                                                    443,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Let,
                                                    Let,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `top_k_row_span_sum`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `i`,
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
                                                    data: [
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            kind: InheritedSymbolKind::ExplicitParameter {
                                                                ident: `k`,
                                                            },
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                435,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        485,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `top_k_row_span_sum`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                444,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        485,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `i`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                469,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        481,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `j`,
                                                                expr_idx: 10,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 26,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        26,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_block_id: ImplBlockId::Type(
                            TypeImplBlockId {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                        ),
                        ident: `top_k_row_right_mass_sum`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodFnDefn {
                                    id: AssociatedItemId {
                                        impl_block_id: ImplBlockId::Type(
                                            TypeImplBlockId {
                                                module_path: `mnist_classifier::connected_component`,
                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        ),
                                        ident: `top_k_row_right_mass_sum`,
                                    },
                                    decl: TypeMethodFnDecl {
                                        id: AssociatedItemId {
                                            impl_block_id: ImplBlockId::Type(
                                                TypeImplBlockId {
                                                    module_path: `mnist_classifier::connected_component`,
                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                            ident: `top_k_row_right_mass_sum`,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ident: `top_k_row_right_mass_sum`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_block_id: ImplBlockId::Type(
                                                    TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                ident: `top_k_row_right_mass_sum`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        parent_ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ident: `top_k_row_right_mass_sum`,
                                                        item_kind: MethodFn,
                                                    },
                                                ),
                                            ),
                                            impl_block: ImplBlock::Type(
                                                TypeImplBlock {
                                                    id: TypeImplBlockId {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ast_idx: 124,
                                                    impl_token: ImplToken {
                                                        token_idx: TokenIdx(
                                                            77,
                                                        ),
                                                    },
                                                    ty_expr: 0,
                                                    body: ArenaIdxRange(
                                                        67..79,
                                                    ),
                                                },
                                            ),
                                            ast_idx: 76,
                                            ident: `top_k_row_right_mass_sum`,
                                            associated_item_kind: TypeItem(
                                                MethodFn,
                                            ),
                                            visibility: Visibility::PubUnder(
                                                `mnist_classifier::connected_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 76,
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
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                            78,
                                                                        ),
                                                                        ident: `ConnectedComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                            ),
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
                                                            impl_block_id: ImplBlockId::Type(
                                                                TypeImplBlockId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            ident: `top_k_row_right_mass_sum`,
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
                                                                490,
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
                                                                493,
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
                                                        data: [
                                                            PatternExpr::Ident {
                                                                modifier: None,
                                                                ident_token: IdentToken {
                                                                    ident: `k`,
                                                                    token_idx: TokenIdx(
                                                                        488,
                                                                    ),
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    pattern_infos: [
                                                        Parameter,
                                                    ],
                                                    pattern_symbol_maps: [
                                                        [
                                                            (
                                                                `k`,
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
                                                                access_start: TokenIdx(
                                                                    489,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSymbolVariant::ExplicitParameter {
                                                                    ident: `k`,
                                                                    pattern_symbol_idx: 0,
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: True,
                                                    pattern_ty_constraints: [
                                                        ExplicitParameter {
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
                                        explicit_parameter_decl_list: ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    487,
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
                                                            489,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            rpar: RightParenthesisToken(
                                                TokenIdx(
                                                    491,
                                                ),
                                            ),
                                        },
                                        curry_token: Some(
                                            CurryToken(
                                                TokenIdx(
                                                    492,
                                                ),
                                            ),
                                        ),
                                        return_ty: Some(
                                            ReturnTypeExpr {
                                                expr: 1,
                                            },
                                        ),
                                        eol_colon: EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    494,
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
                                                                        DeclRegionPath::ImplBlock(
                                                                            ImplBlockId::Type(
                                                                                TypeImplBlockId {
                                                                                    module_path: `mnist_classifier::connected_component`,
                                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                                    78,
                                                                                ),
                                                                                ident: `ConnectedComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                                    ),
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
                                                                    impl_block_id: ImplBlockId::Type(
                                                                        TypeImplBlockId {
                                                                            module_path: `mnist_classifier::connected_component`,
                                                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                            disambiguator: 0,
                                                                        },
                                                                    ),
                                                                    ident: `top_k_row_right_mass_sum`,
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
                                                                        490,
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
                                                                        493,
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
                                                                data: [
                                                                    PatternExpr::Ident {
                                                                        modifier: None,
                                                                        ident_token: IdentToken {
                                                                            ident: `k`,
                                                                            token_idx: TokenIdx(
                                                                                488,
                                                                            ),
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            pattern_infos: [
                                                                Parameter,
                                                            ],
                                                            pattern_symbol_maps: [
                                                                [
                                                                    (
                                                                        `k`,
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
                                                                        access_start: TokenIdx(
                                                                            489,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSymbolVariant::ExplicitParameter {
                                                                            ident: `k`,
                                                                            pattern_symbol_idx: 0,
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: True,
                                                            pattern_ty_constraints: [
                                                                ExplicitParameter {
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_block_id: ImplBlockId::Type(
                                                            TypeImplBlockId {
                                                                module_path: `mnist_classifier::connected_component`,
                                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        ident: `top_k_row_right_mass_sum`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            499,
                                                        ),
                                                    ),
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            501,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            503,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 1,
                                                        opr: Comparison(
                                                            Greater,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            502,
                                                        ),
                                                        ropd: 2,
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            508,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            515,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            516,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                517,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            519,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 6,
                                                        lbox_token_idx: TokenIdx(
                                                            518,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            7..8,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            520,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            524,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            526,
                                                        ),
                                                        ident: `j`,
                                                        frame_var_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            10,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            528,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::InheritedSymbol {
                                                        ident: `k`,
                                                        token_idx: TokenIdx(
                                                            530,
                                                        ),
                                                        inherited_symbol_idx: 0,
                                                        inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `k`,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 9,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            525,
                                                        ),
                                                        ropd: 10,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 11,
                                                        opr: Closed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            529,
                                                        ),
                                                        ropd: 12,
                                                    },
                                                    Expr::Binary {
                                                        lopd: 13,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            527,
                                                        ),
                                                        ropd: 14,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            534,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 16,
                                                        dot_token_idx: TokenIdx(
                                                            535,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `mask`,
                                                            token_idx: TokenIdx(
                                                                536,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `j`,
                                                        token_idx: TokenIdx(
                                                            538,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            10,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 17,
                                                        lbox_token_idx: TokenIdx(
                                                            537,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            18..19,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            539,
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            532,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 19,
                                                        dot_token_idx: TokenIdx(
                                                            540,
                                                        ),
                                                        ident_token: IdentToken {
                                                            ident: `right_mass`,
                                                            token_idx: TokenIdx(
                                                                541,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            542,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            20..20,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            543,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 20,
                                                        opr: AssignClosed(
                                                            Add,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            533,
                                                        ),
                                                        ropd: 21,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            545,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
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
                                                    Expr::Binary {
                                                        lopd: 23,
                                                        opr: As,
                                                        opr_token_idx: TokenIdx(
                                                            546,
                                                        ),
                                                        ropd: 24,
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            3..9,
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
                                                data: [
                                                    Stmt::Break {
                                                        break_token: BreakToken {
                                                            token_idx: TokenIdx(
                                                                522,
                                                            ),
                                                        },
                                                    },
                                                    Stmt::IfElse {
                                                        if_branch: IfBranch {
                                                            if_token: IfToken {
                                                                token_idx: TokenIdx(
                                                                    514,
                                                                ),
                                                            },
                                                            condition: Ok(
                                                                8,
                                                            ),
                                                            eol_colon: Ok(
                                                                Colon(
                                                                    EolColonToken {
                                                                        token_idx: TokenIdx(
                                                                            521,
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
                                                        else_branch: None,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 22,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                495,
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
                                                            EqToken(
                                                                TokenIdx(
                                                                    498,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            0,
                                                        ),
                                                    },
                                                    Stmt::Assert {
                                                        assert_token: AssertToken {
                                                            token_idx: TokenIdx(
                                                                500,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            3,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                504,
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
                                                            EqToken(
                                                                TokenIdx(
                                                                    507,
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            4,
                                                        ),
                                                    },
                                                    Stmt::ForExt {
                                                        forext_token: ForextToken {
                                                            token_idx: TokenIdx(
                                                                509,
                                                            ),
                                                        },
                                                        eol_colon: Err(
                                                            ExprError::Original(
                                                                OriginalExprError::ExpectEolColon(
                                                                    TokenIdx(
                                                                        510,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: StmtForToken {
                                                            token_idx: TokenIdx(
                                                                523,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                526,
                                                            ),
                                                            frame_var_expr_idx: 10,
                                                            frame_var_ident: `j`,
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        9,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        14,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 2,
                                                        eol_colon: Ok(
                                                            EolToken::Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        531,
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
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                544,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            25,
                                                        ),
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `top_k_row_span_sum`,
                                                                token_idx: TokenIdx(
                                                                    497,
                                                                ),
                                                            },
                                                        },
                                                        PatternExpr::Ident {
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `i`,
                                                                token_idx: TokenIdx(
                                                                    506,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Let,
                                                    Let,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `top_k_row_span_sum`,
                                                            0,
                                                        ),
                                                    ],
                                                    [
                                                        (
                                                            `i`,
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
                                                    data: [
                                                        InheritedSymbol {
                                                            parent_symbol_idx: Current(
                                                                0,
                                                            ),
                                                            kind: InheritedSymbolKind::ExplicitParameter {
                                                                ident: `k`,
                                                            },
                                                        },
                                                    ],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                498,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        548,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `top_k_row_span_sum`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                507,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        548,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `i`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                532,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        544,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `j`,
                                                                expr_idx: 10,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 26,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        26,
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