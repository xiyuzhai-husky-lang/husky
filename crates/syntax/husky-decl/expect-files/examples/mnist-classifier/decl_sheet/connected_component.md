Ok(
    DeclSheet {
        decls: [
            (
                DeclRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ast_idx: 121,
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
                                                                TypePath(`core::num::i32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
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
                                                        24,
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
                                                        28,
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
                                                        32,
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
                                                        36,
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        21,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldPattern {
                                            ident_token: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 171,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    22,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    23,
                                                ),
                                            ),
                                            ty: 0,
                                        },
                                        RegularStructFieldPattern {
                                            ident_token: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 172,
                                                        },
                                                    ),
                                                ),
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
                                        },
                                        RegularStructFieldPattern {
                                            ident_token: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 173,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    30,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    31,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        RegularStructFieldPattern {
                                            ident_token: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 174,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    35,
                                                ),
                                            ),
                                            ty: 3,
                                        },
                                    ],
                                    [
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
                                        CommaToken(
                                            TokenIdx(
                                                33,
                                            ),
                                        ),
                                        CommaToken(
                                            TokenIdx(
                                                37,
                                            ),
                                        ),
                                    ],
                                    Ok(
                                        (),
                                    ),
                                ),
                                rcurl: Ok(
                                    RightCurlyBraceToken(
                                        TokenIdx(
                                            38,
                                        ),
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
                            TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ast_idx: 122,
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
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrefixOpn {
                                                    opr: Ref,
                                                    opr_token_idx: TokenIdx(
                                                        48,
                                                    ),
                                                    opd: 0,
                                                },
                                                Expr::NewBoxList {
                                                    caller: None,
                                                    lbox_token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                },
                                                Expr::PrefixOpn {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        47,
                                                    ),
                                                    opd: 1,
                                                },
                                                Expr::Application {
                                                    function: 2,
                                                    argument: 3,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        49,
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        42,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldPattern {
                                            ident_token: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 176,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    44,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    [
                                        CommaToken(
                                            TokenIdx(
                                                50,
                                            ),
                                        ),
                                    ],
                                    Ok(
                                        (),
                                    ),
                                ),
                                rcurl: Ok(
                                    RightCurlyBraceToken(
                                        TokenIdx(
                                            51,
                                        ),
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
                            FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
                                ast_idx: 123,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
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
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrefixOpn {
                                                    opr: Ref,
                                                    opr_token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                    opd: 0,
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
                                                Expr::PrefixOpn {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                    opd: 2,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        58,
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
                                                        62,
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
                                                            ident: `ct`,
                                                            token_idx: TokenIdx(
                                                                55,
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
                                                                    value: 179,
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
                                                        ident: `ct`,
                                                        access_start: TokenIdx(
                                                            56,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                RegularParameter {
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                parameter_decl_list: Ok(
                                    RegularParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                54,
                                            ),
                                        ),
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        56,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    59,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken(
                                        TokenIdx(
                                            60,
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 3,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            63,
                                        ),
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
                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Type(
                        TypeDecl::RegularStruct(
                            RegularStructTypeDecl {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ast_idx: 124,
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
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                83,
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                lcurl: LeftCurlyBraceToken(
                                    TokenIdx(
                                        80,
                                    ),
                                ),
                                field_comma_list: (
                                    [
                                        RegularStructFieldPattern {
                                            ident_token: IdentifierToken {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 182,
                                                        },
                                                    ),
                                                ),
                                                token_idx: TokenIdx(
                                                    81,
                                                ),
                                            },
                                            colon: ColonToken(
                                                TokenIdx(
                                                    82,
                                                ),
                                            ),
                                            ty: 0,
                                        },
                                    ],
                                    [],
                                    Ok(
                                        (),
                                    ),
                                ),
                                rcurl: Ok(
                                    RightCurlyBraceToken(
                                        TokenIdx(
                                            84,
                                        ),
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
                            FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
                                ast_idx: 126,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
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
                                                                TypePath(`core::raw_bits::r32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Alien`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Alien`),
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
                                                        563,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        567,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        570,
                                                    ),
                                                    ident: `r32`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                                        ),
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
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                561,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                565,
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
                                                                    value: 28,
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
                                                                    value: 210,
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
                                                        ident: `a`,
                                                        access_start: TokenIdx(
                                                            562,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        ident: `x`,
                                                        access_start: TokenIdx(
                                                            566,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
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
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                parameter_decl_list: Ok(
                                    RegularParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                560,
                                            ),
                                        ),
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        562,
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
                                                        566,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [
                                            CommaToken(
                                                TokenIdx(
                                                    564,
                                                ),
                                            ),
                                        ],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    568,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken(
                                        TokenIdx(
                                            569,
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 2,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            571,
                                        ),
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
                            FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Decl::Form(
                        FormDecl::Function(
                            FunctionDecl {
                                path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                                ast_idx: 127,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                649,
                                                            ),
                                                            ident: `BinaryImage28`,
                                                        },
                                                    ),
                                                ),
                                                Expr::NewBoxList {
                                                    caller: None,
                                                    lbox_token_idx: TokenIdx(
                                                        652,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        653,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Application {
                                                    function: 1,
                                                    argument: 2,
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        654,
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
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `img`,
                                                            token_idx: TokenIdx(
                                                                647,
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
                                                                    value: 214,
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
                                                        ident: `img`,
                                                        access_start: TokenIdx(
                                                            648,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::RegularParameter {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
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
                                                expr: 3,
                                            },
                                        ],
                                    },
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                parameter_decl_list: Ok(
                                    RegularParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                646,
                                            ),
                                        ),
                                        parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        648,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rpar: Ok(
                                            RightParenthesisToken(
                                                TokenIdx(
                                                    650,
                                                ),
                                            ),
                                        ),
                                    },
                                ),
                                curry_token: Ok(
                                    CurryToken(
                                        TokenIdx(
                                            651,
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    OutputTypeExpr {
                                        expr: 3,
                                    },
                                ),
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            655,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DeclRegionPath::Impl(
                    ImplId {
                        module_path: `mnist_classifier::connected_component`,
                        impl_kind: ImplKind::Type {
                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                ),
                Ok(
                    Decl::Impl(
                        ImplDecl::Type(
                            TypeImplDecl {
                                ast_idx: 125,
                                im: Impl {
                                    id: ImplId {
                                        module_path: `mnist_classifier::connected_component`,
                                        impl_kind: ImplKind::Type {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        },
                                        disambiguator: 0,
                                    },
                                    ast_idx: 125,
                                    body: ArenaIdxRange(
                                        67..79,
                                    ),
                                    variant: ImplVariant::Type {
                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        85,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            87,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Impl(
                                                ImplId {
                                                    module_path: `mnist_classifier::connected_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        86,
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
                DeclRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `raw_contours`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `raw_contours`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `raw_contours`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `raw_contours`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 67,
                                        ident: `raw_contours`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 67,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `raw_contours`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::NewBoxList {
                                                        caller: None,
                                                        lbox_token_idx: TokenIdx(
                                                            91,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            0..0,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                ),
                                                            ),
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
                                                            93,
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
                                                allow_self_value: True,
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                90,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 2,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                94,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `eff_holes`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `eff_holes`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `eff_holes`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `eff_holes`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 68,
                                        ident: `eff_holes`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 68,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `eff_holes`,
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
                                                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                            102,
                                                        ),
                                                        ident: `EffHoles`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                                            ),
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                101,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                103,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `max_hole_ilen`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `max_hole_ilen`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `max_hole_ilen`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `max_hole_ilen`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 69,
                                        ident: `max_hole_ilen`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 69,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `max_hole_ilen`,
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
                                                            163,
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                162,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                164,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `max_row_span`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `max_row_span`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `max_row_span`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `max_row_span`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 70,
                                        ident: `max_row_span`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 70,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `max_row_span`,
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
                                                            215,
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                214,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                216,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `row_span_sum`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `row_span_sum`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `row_span_sum`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `row_span_sum`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 71,
                                        ident: `row_span_sum`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 71,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `row_span_sum`,
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
                                                            253,
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                252,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                254,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `distribution`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `distribution`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `distribution`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `distribution`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 72,
                                        ident: `distribution`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 72,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `distribution`,
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
                                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                            286,
                                                        ),
                                                        ident: `ConnectedComponentDistribution`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                            ),
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                285,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                287,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `upper_mass`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `upper_mass`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `upper_mass`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `upper_mass`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 73,
                                        ident: `upper_mass`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 73,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `upper_mass`,
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
                                                            408,
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                407,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                409,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `lower_mass`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Memo(
                                TypeMemoDecl {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `lower_mass`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `lower_mass`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `lower_mass`,
                                                    ty_item_kind: Memo,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 74,
                                        ident: `lower_mass`,
                                        associated_item_kind: TypeItem(
                                            Memo,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    ast_idx: 74,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `lower_mass`,
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
                                                            420,
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
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                419,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                421,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `top_k_row_span_sum`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `top_k_row_span_sum`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `top_k_row_span_sum`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 75,
                                        ident: `top_k_row_span_sum`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `top_k_row_span_sum`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    ast_idx: 75,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `top_k_row_span_sum`,
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
                                                            434,
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
                                                            437,
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
                                                                ident: `k`,
                                                                token_idx: TokenIdx(
                                                                    432,
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
                                                                        value: 205,
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
                                                            ident: `k`,
                                                            access_start: TokenIdx(
                                                                433,
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                    parameter_decl_list: Ok(
                                        RegularParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    431,
                                                ),
                                            ),
                                            parameters: [
                                                RegularParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            433,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        435,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                436,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                438,
                                            ),
                                        ),
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
                        impl_id: ImplId {
                            module_path: `mnist_classifier::connected_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `top_k_row_right_mass_sum`,
                    },
                ),
                Ok(
                    Decl::AssociatedItem(
                        AssociatedItemDecl::TypeItem(
                            TypeItemDecl::Method(
                                TypeMethodDecl {
                                    associated_item: AssociatedItem {
                                        id: AssociatedItemId {
                                            impl_id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ident: `top_k_row_right_mass_sum`,
                                        },
                                        path: Some(
                                            AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                    ident: `top_k_row_right_mass_sum`,
                                                    ty_item_kind: Method,
                                                },
                                            ),
                                        ),
                                        im: Impl {
                                            id: ImplId {
                                                module_path: `mnist_classifier::connected_component`,
                                                impl_kind: ImplKind::Type {
                                                    ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                },
                                                disambiguator: 0,
                                            },
                                            ast_idx: 125,
                                            body: ArenaIdxRange(
                                                67..79,
                                            ),
                                            variant: ImplVariant::Type {
                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            },
                                        },
                                        ast_idx: 76,
                                        ident: `top_k_row_right_mass_sum`,
                                        associated_item_kind: TypeItem(
                                            Method,
                                        ),
                                        accessibility: Accessibility::PublicUnder(
                                            `mnist_classifier::connected_component`,
                                        ),
                                        is_generic: false,
                                    },
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            ident: `top_k_row_right_mass_sum`,
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
                                                            DeclRegionPath::Impl(
                                                                ImplId {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    impl_kind: ImplKind::Type {
                                                                        ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                                        86,
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
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::connected_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `top_k_row_right_mass_sum`,
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
                                                            497,
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
                                                            500,
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
                                                                ident: `k`,
                                                                token_idx: TokenIdx(
                                                                    495,
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
                                                                        value: 205,
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
                                                            ident: `k`,
                                                            access_start: TokenIdx(
                                                                496,
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                    parameter_decl_list: Ok(
                                        RegularParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    494,
                                                ),
                                            ),
                                            parameters: [
                                                RegularParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            496,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        498,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                499,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                501,
                                            ),
                                        ),
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